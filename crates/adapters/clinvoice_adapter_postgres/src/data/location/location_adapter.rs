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
			"INSERT INTO locations (name) VALUES ($1) RETURNING id;",
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
	use clinvoice_adapter::data::Initializable;

	use super::{LocationAdapter, PostgresLocation};
	use crate::data::{util, PostgresSchema};

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let usa = PostgresLocation::create_inner(
			&mut connection,
			&earth,
			"USA".into()
		)
		.await
		.unwrap();

		let arizona = PostgresLocation::create_inner(
			&mut connection,
			&usa,
			"Arizona".into()
		)
		.await
		.unwrap();

		let phoenix = PostgresLocation::create_inner(
			&mut connection,
			&arizona,
			"Phoenix".into()
		)
		.await
		.unwrap();

		let database_earth = sqlx::query!("SELECT * FROM locations WHERE id = $1", earth.id).await.unwrap();

		assert_eq!(earth.id, database_earth.id);
		assert_eq!(earth.name, database_earth.name);
		assert_eq!(earth.outer_id, database_earth.outer_id);

		let database_usa = sqlx::query!("SELECT * FROM locations WHERE id = $1", usa.id).await.unwrap();

		assert_eq!(usa.outer_id, Some(earth.id));

		let database_arizona = sqlx::query!("SELECT * FROM locations WHERE id = $1", arizona.id).await.unwrap();

		assert_eq!(usa.outer_id, Some(earth.id));

		let database_phoenix = sqlx::query!("SELECT * FROM locations WHERE id = $1", phoenix.id).await.unwrap();

		assert_eq!(usa.outer_id, Some(earth.id));
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
