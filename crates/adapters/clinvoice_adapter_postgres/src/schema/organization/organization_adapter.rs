use clinvoice_adapter::{schema::OrganizationAdapter, WriteWhereClause};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::{Location, Organization};
use futures::TryStreamExt;
use sqlx::{PgPool, Result};

use super::{columns::PgOrganizationColumns, PgOrganization};
use crate::{schema::PgLocation, PgSchema as Schema};

#[async_trait::async_trait]
impl OrganizationAdapter for PgOrganization
{
	async fn create(connection: &PgPool, location: Location, name: String) -> Result<Organization>
	{
		let row = sqlx::query!(
			"INSERT INTO organizations (location_id, name) VALUES ($1, $2) RETURNING id;",
			location.id,
			name
		)
		.fetch_one(connection)
		.await?;

		Ok(Organization {
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
		let id_match = PgLocation::retrieve_matching_ids(connection, &match_condition.location);

		let mut query = String::from(
			"SELECT O.id, O.location_id, O.name
			FROM organizations O
			JOIN locations L ON (L.id = O.location_id)",
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

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move { COLUMNS.row_to_view(connection, &row).await })
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::schema::LocationAdapter;
	use clinvoice_match::{Match, MatchLocation, MatchOrganization, MatchOuterLocation};

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

		let organization =
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into())
				.await
				.unwrap();

		let row = sqlx::query!(
			"SELECT * FROM organizations WHERE id = $1;",
			organization.id
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(organization.id, row.id);
		assert_eq!(organization.location.id, earth.id);
		assert_eq!(organization.location.id, row.location_id);
		assert_eq!(organization.name, row.name);
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

		let (some_organization, some_other_organization) = futures::try_join!(
			PgOrganization::create(&connection, arizona, "Some Organization".into()),
			PgOrganization::create(&connection, utah, "Some Other Organizatión".into()),
		)
		.unwrap();

		// Assert ::retrieve gets the right data from the DB
		assert_eq!(
			PgOrganization::retrieve(&connection, MatchOrganization {
				id: some_organization.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[some_organization.clone()],
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
			.as_slice(),
			&[some_organization, some_other_organization],
		);
	}
}
