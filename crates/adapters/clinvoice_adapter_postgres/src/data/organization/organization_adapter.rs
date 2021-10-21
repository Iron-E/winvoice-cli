use clinvoice_adapter::data::OrganizationAdapter;
use clinvoice_data::{views::OrganizationView, Location, Organization};
use clinvoice_query as query;
use sqlx::{Executor, Postgres, Result};

use super::PostgresOrganization;

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
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Organization,
	) -> Result<Vec<OrganizationView>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::data::{Initializable, LocationAdapter};

	use super::{OrganizationAdapter, PostgresOrganization};
	use crate::data::{util, PostgresLocation, PostgresSchema};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let organization = PostgresOrganization::create(&mut connection, &earth, "Some Organization".into())
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
		// TODO: write test
	}
}
