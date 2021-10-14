use clinvoice_adapter::data::PersonAdapter;
use clinvoice_data::Person;
use clinvoice_query as query;
use futures::Stream;
use sqlx::{Executor, Postgres, Result};

use super::PostgresPerson;

#[async_trait::async_trait]
impl PersonAdapter for PostgresPerson
{
	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		name: String,
	) -> Result<Person>
	{
		let row = sqlx::query!("INSERT INTO people (name) VALUES ($1) RETURNING id;", name)
			.fetch_one(connection)
			.await?;

		Ok(Person { id: row.id, name })
	}

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	fn retrieve<'a, E, S>(connection: E, query: &query::Person) -> S
	where
		E: Executor<'a, Database = Postgres>,
		S: Stream<Item = Result<Person>>,
	{
		// sqlx::query("").fetch(&mut connection).await
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::data::Initializable;

	use super::{PersonAdapter, PostgresPerson};
	use crate::data::{util, PostgresSchema};

	#[tokio::test]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();
		assert!(sqlx::query!("SELECT * FROM people;")
			.fetch_optional(&mut connection)
			.await
			.unwrap()
			.is_none());
		let person = PostgresPerson::create(&mut connection, "foo".into())
			.await
			.unwrap();
		let row = sqlx::query!("SELECT * FROM people;")
			.fetch_one(&mut connection)
			.await
			.unwrap();
		assert_eq!(person.id, row.id);
		assert_eq!(person.name, row.name);
	}

	#[tokio::test]
	async fn retrieve()
	{
		// TODO: write test; `SET SCHEMA 'pg_temp';`
	}
}
