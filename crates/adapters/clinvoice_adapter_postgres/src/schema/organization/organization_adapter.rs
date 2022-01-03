use clinvoice_adapter::{schema::OrganizationAdapter, WriteWhereClause};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::{views::OrganizationView, Location, Organization};
use futures::TryStreamExt;
use sqlx::{PgPool, Result, Row};

use super::PgOrganization;
use crate::{schema::PgLocation, PgSchema as Schema};

#[async_trait::async_trait]
impl OrganizationAdapter for PgOrganization
{
	async fn create(connection: &PgPool, location: &Location, name: String) -> Result<Organization>
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
			location_id: location.id,
			name,
		})
	}

	async fn retrieve_view(
		connection: &PgPool,
		match_condition: &MatchOrganization,
	) -> Result<Vec<OrganizationView>>
	{
		let id_match = PgLocation::retrieve_matching_ids(connection, &match_condition.location);
		let mut query = String::from(
			"SELECT O.id, O.location_id, O.name
			FROM organizations O
			JOIN locations L ON (L.id = O.location_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(Default::default(), "O", match_condition, &mut query),
			"L.id",
			&id_match.await?,
			&mut query,
		);
		query.push(';');

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move {
				Ok(OrganizationView {
					id: row.get("id"),
					name: row.get("name"),
					location: PgLocation::retrieve_view_by_id(connection, row.get("location_id"))
						.await?,
				})
			})
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::borrow::Cow::Owned;

	use clinvoice_adapter::schema::LocationAdapter;
	use clinvoice_match::{Match, MatchLocation, MatchOrganization, MatchOuterLocation};
	use clinvoice_schema::views::{LocationView, OrganizationView};

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
			PgOrganization::create(&connection, &earth, "Some Organization".into())
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
		assert_eq!(organization.location_id, earth.id);
		assert_eq!(organization.location_id, row.location_id);
		assert_eq!(organization.name, row.name);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PgLocation::create_inner(&connection, &earth, "USA".into())
			.await
			.unwrap();

		let arizona = PgLocation::create_inner(&connection, &usa, "Arizona".into())
			.await
			.unwrap();

		let utah = PgLocation::create_inner(&connection, &usa, "Utah".into())
			.await
			.unwrap();

		let earth_view = LocationView {
			id: earth.id,
			name: earth.name.clone(),
			outer: None,
		};

		let usa_view = LocationView {
			id: usa.id,
			name: usa.name.clone(),
			outer: Some(earth_view.clone().into()),
		};

		let arizona_view = LocationView {
			id: arizona.id,
			name: arizona.name.clone(),
			outer: Some(usa_view.clone().into()),
		};

		let utah_view = LocationView {
			id: utah.id,
			name: utah.name.clone(),
			outer: Some(usa_view.clone().into()),
		};

		let some_organization =
			PgOrganization::create(&connection, &arizona.into(), "Some Organization".into())
				.await
				.unwrap();
		let some_other_organization =
			PgOrganization::create(&connection, &utah.into(), "Some Other Organizatión".into())
				.await
				.unwrap();

		let some_organization_view = OrganizationView {
			id: some_organization.id,
			name: some_organization.name.clone(),
			location: arizona_view,
		};
		let some_other_organization_view = OrganizationView {
			id: some_other_organization.id,
			name: some_other_organization.name.clone(),
			location: utah_view,
		};

		// Assert ::retrieve_view gets the right data from the DB
		assert_eq!(
			PgOrganization::retrieve_view(&connection, &MatchOrganization {
				id: some_organization_view.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[some_organization_view.clone()],
		);

		assert_eq!(
			PgOrganization::retrieve_view(&connection, &MatchOrganization {
				location: MatchLocation {
					outer: MatchOuterLocation::Some(
						MatchLocation {
							id: Match::InRange(Owned(usa.id - 1), Owned(usa.id + 1)),
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
			&[some_organization_view, some_other_organization_view],
		);
	}
}
