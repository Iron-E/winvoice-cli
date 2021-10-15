use clinvoice_adapter::data::LocationAdapter;
use clinvoice_data::{views::LocationView, Location};
use clinvoice_query as query;
use sqlx::{Acquire, Executor, Postgres, Result};

use super::PostgresLocation;

#[async_trait::async_trait]
impl LocationAdapter for PostgresLocation
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		name: String,
	) -> Result<Location>
	{
		todo!()
	}

	async fn create_inner(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		outer: &Location,
		name: String,
	) -> Result<Location>
	{
		todo!()
	}

	async fn retrieve(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Location,
	) -> Result<Vec<Location>>
	{
		todo!()
	}

	async fn retrieve_outers(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		location: &Location,
	) -> Result<Vec<Location>>
	{
		todo!()
	}

	// WARN: `Might need `Acquire` or `&mut Transaction` depending on how recursive views work
	async fn retrieve_view(
		connection: impl 'async_trait + Acquire<'_, Database = Postgres> + Send,
		query: &query::Location,
	) -> Result<Vec<LocationView>>
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
		// TODO: write test + `create_inner`
	}

	#[tokio::test]
	async fn retrieve()
	{
		// TODO: write test + `retrieve_outers`
	}

	#[tokio::test]
	async fn retrieve_outers()
	{
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
