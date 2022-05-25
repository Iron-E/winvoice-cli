use clinvoice_adapter::{
	schema::{ContactInfoAdapter, OrganizationAdapter},
	WriteWhereClause,
};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::{ContactKind, Id, Location, Organization};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{PgPool, Result, Row};

use super::{columns::PgOrganizationColumns, PgOrganization};
use crate::{
	schema::{PgContactInfo, PgLocation},
	PgSchema as Schema,
};

#[async_trait::async_trait]
impl OrganizationAdapter for PgOrganization
{
	async fn create(
		connection: &PgPool,
		contact_info: Vec<(bool, ContactKind, String)>,
		location: Location,
		name: String,
	) -> Result<Organization>
	{
		let mut transaction = connection.begin().await?;

		let row = sqlx::query!(
			"INSERT INTO organizations (location_id, name) VALUES ($1, $2) RETURNING id;",
			location.id,
			name
		)
		.fetch_one(connection)
		.await?;

		let contact_info_db = PgContactInfo::create(&mut transaction, contact_info, row.id).await?;

		transaction.commit().await?;

		Ok(Organization {
			contact_info: contact_info_db,
			id: row.id,
			location,
			name,
		})
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: MatchOrganization,
	) -> Result<Vec<Organization>>
	{
		let contact_info_fut =
			PgContactInfo::retrieve(connection, match_condition.contact_info.clone());
		let id_match = PgLocation::retrieve_matching_ids(connection, &match_condition.location);

		let mut query = String::from(
			"SELECT
				O.id,
				O.location_id,
				O.name
			FROM organizations O",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(Default::default(), "O", &match_condition, &mut query),
			"L.id",
			&id_match.await?,
			&mut query,
		);
		query.push(';');

		const COLUMNS: PgOrganizationColumns<'static> = PgOrganizationColumns {
			id: "id",
			location_id: "location_id",
			name: "name",
		};

		let contact_info = contact_info_fut.await?;
		sqlx::query(&query)
			.fetch(connection)
			.try_filter_map(|row| async move {
				if let Some(c) = contact_info.get(&row.get::<Id, _>(COLUMNS.id))
				{
					return COLUMNS
						.row_to_view(connection, c.clone(), &row)
						.map_ok(Some)
						.await;
				}

				// If `PgContactInfo::retrieve` does not match,
				// then the whole `employee` does not match.
				Ok(None)
			})
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashSet;

	use clinvoice_adapter::schema::LocationAdapter;
	use clinvoice_match::{Match, MatchLocation, MatchOrganization, MatchOuterLocation};
	use clinvoice_schema::{Contact, ContactKind};
	use futures::TryStreamExt;

	use super::{OrganizationAdapter, PgOrganization};
	use crate::schema::{util, PgLocation};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization = PgOrganization::create(
			&connection,
			vec![
				(true, ContactKind::Address(earth), "Office".into()),
				(
					true,
					ContactKind::Email("foo@bar.io".into()),
					"Work Email".into(),
				),
				(
					true,
					ContactKind::Phone("555 223 5039".into()),
					"Office's Email".into(),
				),
			],
			earth.clone(),
			"Some Organization".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!(
			"SELECT * FROM organizations WHERE id = $1;",
			organization.id
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		let contact_info_row = async {
			let connection_borrow = &connection;
			sqlx::query!(
				"SELECT * FROM contact_information WHERE organization_id = $1;",
				organization.id
			)
			.fetch(connection_borrow)
			.and_then(|row| async move {
				Ok(Contact {
					organization_id: row.organization_id,
					export: row.export,
					label: row.label,
					kind: match row
						.email
						.map(ContactKind::Email)
						.or_else(|| row.phone.map(ContactKind::Phone))
					{
						Some(k) => k,
						_ => ContactKind::Address(
							PgLocation::retrieve_by_id(connection_borrow, row.address_id.unwrap()).await?,
						),
					},
				})
			})
			.try_collect::<Vec<_>>()
			.await
			.unwrap()
		};

		// Assert ::create writes accurately to the DB
		assert_eq!(organization.id, row.id);
		assert_eq!(organization.location.id, earth.id);
		assert_eq!(organization.location.id, row.location_id);
		assert_eq!(organization.name, row.name);
		assert_eq!(organization.contact_info, contact_info_row.await);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PgLocation::create_inner(&connection, earth, "USA".into())
			.await
			.unwrap();

		let (arizona, utah) = futures::try_join!(
			PgLocation::create_inner(&connection, usa.clone(), "Arizona".into()),
			PgLocation::create_inner(&connection, usa.clone(), "Utah".into()),
		)
		.unwrap();

		let (organization, organization2) = futures::try_join!(
			PgOrganization::create(
				&connection,
				vec![
					(false, ContactKind::Address(utah), "Remote Office".into()),
					(
						true,
						ContactKind::Email("foo@bar.io".into()),
						"Work Email".into(),
					),
					(
						true,
						ContactKind::Phone("555 223 5039".into()),
						"Office's Phone".into(),
					),
				],
				arizona.clone(),
				"Some Organization".into(),
			),
			PgOrganization::create(
				&connection,
				Default::default(),
				utah.clone(),
				"Some Other Organizatión".into(),
			),
		)
		.unwrap();

		// Assert ::retrieve gets the right data from the DB
		assert_eq!(
			PgOrganization::retrieve(&connection, MatchOrganization {
				id: organization.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[organization.clone()],
		);

		assert_eq!(
			PgOrganization::retrieve(&connection, MatchOrganization {
				location: MatchLocation {
					outer: MatchOuterLocation::Some(
						MatchLocation {
							id: Match::InRange(usa.id - 1, usa.id + 1),
							name: usa.name.into(),
							..Default::default()
						}
						.into()
					),
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>(),
			[organization, organization2].into_iter().collect(),
		);
	}
}
