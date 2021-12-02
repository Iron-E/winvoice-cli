use clinvoice_adapter::{
	schema::OrganizationAdapter,
	WriteContext,
	WriteFromClause,
	WriteJoinClause,
	WriteSelectClause,
	WriteWhereClause,
};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::{views::OrganizationView, Location, Organization};
use sqlx::{Acquire, Executor, Postgres, Result, Row};

use super::PostgresOrganization;
use crate::{schema::PostgresLocation, PostgresSchema as Schema};

#[async_trait::async_trait]
impl OrganizationAdapter for PostgresOrganization
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		location: &Location,
		name: String,
	) -> Result<Organization>
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
		connection: impl 'async_trait + Acquire<'_, Database = Postgres> + Send,
		match_condition: &MatchOrganization,
	) -> Result<Vec<OrganizationView>>
	{
		let mut transaction = connection.begin().await?;
		let mut query = Schema::write_select_clause([]);
		Schema::write_from_clause(&mut query, "organizations", "O");
		Schema::write_join_clause(&mut query, "", "locations", "L", "id", "O.location_id").unwrap();
		Schema::write_where_clause(
			WriteContext::BeforeWhereClause,
			"O",
			match_condition,
			&mut query,
		);
		query.push(';');

		let selected = sqlx::query(&query).fetch_all(&mut transaction).await?;
		let mut output = Vec::with_capacity(selected.len());

		// NOTE: because of the mutable borrow here, we need to use a `for` rather than a fancy
		//       closure :(
		for row in selected
		{
			output.push(OrganizationView {
				id: row.get("id"),
				name: row.get("name"),
				location: PostgresLocation::retrieve_view_by_id(&mut transaction, row.get("id"))
					.await?,
			});
		}

		transaction.rollback().await?;
		Ok(output)
	}
}

#[cfg(test)]
mod tests
{
	use std::borrow::Cow::{Borrowed, Owned};
	use clinvoice_adapter::{schema::LocationAdapter, Initializable};
	use clinvoice_match::{MatchOrganization, Match, MatchLocation};
	use clinvoice_schema::views::{LocationView, OrganizationView};

	use super::{OrganizationAdapter, PostgresOrganization};
	use crate::{
		schema::{util, PostgresLocation},
		PostgresSchema,
	};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&mut connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let row = sqlx::query!("SELECT * FROM organizations;")
			.fetch_one(&mut connection)
			.await
			.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(organization.id, row.id);
		assert_eq!(organization.location_id, earth.id);
		assert_eq!(organization.location_id, row.id);
		assert_eq!(organization.name, row.name);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let usa = PostgresLocation::create_inner(&mut connection, &earth, "USA".into())
			.await
			.unwrap();

		let arizona = PostgresLocation::create_inner(&mut connection, &usa, "Arizona".into())
			.await
			.unwrap();

		let utah = PostgresLocation::create_inner(&mut connection, &usa, "Utah".into())
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

		let some_organization = PostgresOrganization::create(&mut connection, &arizona.into(), "Some Organization".into()).await.unwrap();
		let some_organization_view = OrganizationView {
			id: some_organization.id,
			name: some_organization.name.clone(),
			location: arizona_view,
		};

		// Assert ::create writes accurately to the DB
		assert_eq!(
			&[some_organization_view],
			PostgresOrganization::retrieve_view(&mut connection, &MatchOrganization {
				id: Match::EqualTo(Owned(some_organization.id)),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice()
		);
	}
}
