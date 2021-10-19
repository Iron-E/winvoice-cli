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
		location: Location,
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
	#[tokio::test]
	async fn create()
	{
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
