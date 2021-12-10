use clinvoice_adapter::{schema::OrganizationAdapter, WriteWhereClause};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::{views::OrganizationView, Location, Organization};
use futures::TryStreamExt;
use sqlx::{PgPool, Result, Row};

use super::PostgresOrganization;
use crate::{schema::PostgresLocation, PostgresSchema as Schema};

#[async_trait::async_trait]
impl OrganizationAdapter for PostgresOrganization
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
		let id_match = PostgresLocation::retrieve_matching_ids(connection, &match_condition.location);
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
					location: PostgresLocation::retrieve_view_by_id(connection, row.get("location_id"))
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
	use clinvoice_adapter::schema::LocationAdapter;
	use clinvoice_match::MatchOrganization;
	use clinvoice_schema::views::{LocationView, OrganizationView};

	use super::{OrganizationAdapter, PostgresOrganization};
	use crate::schema::{util, PostgresLocation};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PostgresLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&connection, &earth, "Some Organization".into())
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

		let earth = PostgresLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PostgresLocation::create_inner(&connection, &earth, "USA".into())
			.await
			.unwrap();

		let arizona = PostgresLocation::create_inner(&connection, &usa, "Arizona".into())
			.await
			.unwrap();

		let utah = PostgresLocation::create_inner(&connection, &usa, "Utah".into())
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
			PostgresOrganization::create(&connection, &arizona.into(), "Some Organization".into())
				.await
				.unwrap();
		let some_organization_view = OrganizationView {
			id: some_organization.id,
			name: some_organization.name.clone(),
			location: arizona_view,
		};

		// Assert ::retrieve_view gets the right data from the DB
		assert_eq!(
			&[some_organization_view],
			PostgresOrganization::retrieve_view(&connection, &MatchOrganization {
				id: some_organization.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice()
		);

		// TODO: make more organizations
	}
}
