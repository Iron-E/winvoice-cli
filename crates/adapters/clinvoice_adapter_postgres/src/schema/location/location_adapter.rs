use clinvoice_adapter::{
	schema::LocationAdapter,
	WriteContext,
	WriteFromClause,
	WriteSelectClause,
	WriteWhereClause,
};
use clinvoice_match::MatchLocation;
use clinvoice_schema::{views::LocationView, Id, Location};
use futures::{future, TryStreamExt};
use sqlx::{Acquire, Executor, Postgres, Result, Row};

use super::PostgresLocation;
use crate::PostgresSchema as Schema;

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
		match_condition: &MatchLocation,
	) -> Result<Vec<LocationView>>
	{
		let mut transaction = connection.begin().await?;
		let mut query = Schema::write_select_clause([]);
		Schema::write_from_clause(&mut query, "locations", "L");
		Schema::write_where_clause(
			WriteContext::BeforeWhereClause,
			"L",
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
			output.push(
				sqlx::query!(
					"WITH RECURSIVE location_view AS
					(
						SELECT id, name, outer_id FROM locations WHERE id = $1
						UNION
						SELECT L.id, L.name, L.outer_id FROM locations L JOIN location_view V ON (L.id = V.outer_id)
					) SELECT * FROM location_view ORDER BY id;",
					row.get::<Id, _>("id")
				)
				.fetch(&mut transaction)
				.try_fold(None, |previous: Option<LocationView>, view_row| {
					future::ok(Some(LocationView {
						id: view_row
							.id
							.expect("`locations` table should have non-null ID"),
						name: view_row
							.name
							.expect("`locations` table should have non-null name"),
						outer: previous.map(Box::new),
					}))
				})
				.await?
				.expect("A database object failed to be returned by recursive query"),
			);
		}

		transaction.rollback().await?;
		Ok(output)
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::Initializable;

	use super::{LocationAdapter, PostgresLocation};
	use crate::{schema::util, PostgresSchema};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
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

		// Assert ::create_inner works when `outer_id` has already been used for another `Location`
		assert!(
			PostgresLocation::create_inner(&mut connection, &usa, "Utah".into())
				.await
				.is_ok()
		);

		macro_rules! select {
			($id:expr) => {
				sqlx::query!("SELECT * FROM locations WHERE id = $1", $id)
					.fetch_one(&mut connection)
					.await
					.unwrap()
			};
		}

		let database_earth = select!(earth.id);

		// Assert ::create writes accurately to the DB
		assert_eq!(earth.id, database_earth.id);
		assert_eq!(earth.name, database_earth.name);
		assert_eq!(earth.outer_id, None);
		assert_eq!(earth.outer_id, database_earth.outer_id);

		let database_usa = select!(usa.id);

		// Assert ::create_inner writes accurately to the DB when `outer_id` is `None`
		assert_eq!(usa.id, database_usa.id);
		assert_eq!(usa.name, database_usa.name);
		assert_eq!(usa.outer_id, Some(earth.id));
		assert_eq!(usa.outer_id, database_usa.outer_id);

		let database_arizona = select!(arizona.id);

		// Assert ::create_inner writes accurately to the DB when `outer_id` is `Some(…)`
		assert_eq!(arizona.id, database_arizona.id);
		assert_eq!(arizona.name, database_arizona.name);
		assert_eq!(arizona.outer_id, Some(usa.id));
		assert_eq!(arizona.outer_id, database_arizona.outer_id);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
		// TODO: profile this
	}
}
