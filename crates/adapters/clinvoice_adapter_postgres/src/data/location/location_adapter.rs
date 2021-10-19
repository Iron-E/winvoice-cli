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
		let row = sqlx::query!(
			"INSERT INTO locations (name, outer_id) VALUES ($1, NULL) RETURNING id;",
			name
		)
		.fetch_one(connection)
		.await?;

		Ok(Location {
			id: row.id,
			name,
			outer_id: None,
		})
	}

	async fn create_inner(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		outer: &Location,
		name: String,
	) -> Result<Location>
	{
		let row = sqlx::query!(
			"INSERT INTO locations (name, outer_id) VALUES ($1, $2) RETURNING id;",
			name,
			outer.id
		)
		.fetch_one(connection)
		.await?;

		Ok(Location {
			id: row.id,
			name,
			outer_id: Some(outer.id),
		})
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
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
