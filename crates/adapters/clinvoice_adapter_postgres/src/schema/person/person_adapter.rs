use clinvoice_adapter::{schema::PersonAdapter, WriteWhereClause};
use clinvoice_match::MatchPerson;
use clinvoice_schema::{views::PersonView, Person};
use futures::stream::TryStreamExt;
use sqlx::{PgPool, Result};

use super::{columns::PgPersonColumns, PgPerson};
use crate::PgSchema as Schema;

#[async_trait::async_trait]
impl PersonAdapter for PgPerson
{
	async fn create(connection: &PgPool, name: String) -> Result<Person>
	{
		let row = sqlx::query!("INSERT INTO people (name) VALUES ($1) RETURNING id;", name)
			.fetch_one(connection)
			.await?;

		Ok(Person { id: row.id, name })
	}

	async fn retrieve_view(
		connection: &PgPool,
		match_condition: MatchPerson,
	) -> Result<Vec<PersonView>>
	{
		let mut query = String::from("SELECT * FROM people P");
		Schema::write_where_clause(Default::default(), "P", &match_condition, &mut query);
		query.push(';');

		const COLUMNS: PgPersonColumns<'static> = PgPersonColumns {
			id: "id",
			name: "name",
		};

		sqlx::query(&query)
			.fetch(connection)
			.map_ok(|row| COLUMNS.row_to_view(&row))
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use super::{PersonAdapter, PgPerson};
	use crate::schema::util;

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let person = PgPerson::create(&connection, "foo".into()).await.unwrap();

		let row = sqlx::query!("SELECT * FROM people WHERE id = $1;", person.id)
			.fetch_one(&connection)
			.await
			.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(person.id, row.id);
		assert_eq!(person.name, row.name);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		// TODO: write test; `SET SCHEMA 'pg_temp';`
	}
}
