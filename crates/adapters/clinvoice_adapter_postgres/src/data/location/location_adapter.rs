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

		// Testing ::create
		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		// Testing ::create_inner when `outer_id` is `None`
		let usa = PostgresLocation::create_inner(
			&mut connection,
			&earth,
			"USA".into(),
		)
		.await
		.unwrap();

		// Testing ::create_inner when `outer_id` is `Some(…)`
		let arizona = PostgresLocation::create_inner(
			&mut connection,
			&usa,
			"Arizona".into(),
		)
		.await
		.unwrap();

		macro_rules! select {
			($id:expr) => {
				sqlx::query!("SELECT * FROM locations WHERE id = $1", $id).fetch_one(&mut connection).await.unwrap()
			}
		}

		let database_earth = select!(earth.id);

		assert_eq!(earth.id, database_earth.id);
		assert_eq!(earth.name, database_earth.name);
		assert_eq!(earth.outer_id, None);
		assert_eq!(earth.outer_id, database_earth.outer_id);

		let database_usa = select!(usa.id);

		assert_eq!(usa.id, database_usa.id);
		assert_eq!(usa.name, database_usa.name);
		assert_eq!(usa.outer_id, Some(earth.id));
		assert_eq!(usa.outer_id, database_usa.outer_id);

		let database_arizona = select!(arizona.id);

		assert_eq!(arizona.id, database_arizona.id);
		assert_eq!(arizona.name, database_arizona.name);
		assert_eq!(arizona.outer_id, Some(usa.id));
		assert_eq!(arizona.outer_id, database_arizona.outer_id);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
