use clinvoice_data::views::OrganizationView;
use sqlx::{Executor, Postgres, Result};

use
{
	super::PostgresOrganization,

	clinvoice_adapter::data::OrganizationAdapter,
	clinvoice_data::{Location, Organization},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl OrganizationAdapter for PostgresOrganization
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		location: Location,
		name: String,
	) -> Result<Organization>
	{
		todo!()
	}

	async fn retrieve(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Organization,
	) -> Result<Vec<Organization>>
	{
		todo!()
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
	async fn retrieve()
	{
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
