use clinvoice_adapter::{
	schema::PersonAdapter,
	WriteSqlFromClause,
	WriteSqlSelectClause,
	WriteSqlWhereClause,
};
use clinvoice_query as query;
use clinvoice_schema::{views::PersonView, Person};
use futures::stream::TryStreamExt;
use sqlx::{postgres::Postgres, Executor, Result, Row};

use super::PostgresPerson;
use crate::PostgresSchema;

#[async_trait::async_trait]
impl PersonAdapter for PostgresPerson
{
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

	async fn retrieve_view(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Person,
	) -> Result<Vec<PersonView>>
	{
		let mut sql = PostgresSchema::write_sql_select_clause(["*"]);
		PostgresSchema::write_sql_from_clause(&mut sql, "people", "");

		const PREFIX_WHERE_CLAUSE: Option<&str> = Some("WHERE");

		PostgresSchema::write_sql_where_clause(
			if PostgresSchema::write_sql_where_clause(PREFIX_WHERE_CLAUSE, "id", &query.id, &mut sql)
			{
				None
			}
			else
			{
				PREFIX_WHERE_CLAUSE
			},
			"name",
			&query.name,
			&mut sql,
		);

		sql.push(';');
		sqlx::query(&sql)
			.fetch(connection)
			.map_ok(|row| PersonView {
				id:   row.get("id"),
				name: row.get("name"),
			})
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::Initializable;

	use super::{PersonAdapter, PostgresPerson};
	use crate::schema::{util, PostgresSchema};

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let person = PostgresPerson::create(&mut connection, "foo".into())
			.await
			.unwrap();

		let row = sqlx::query!("SELECT * FROM people;")
			.fetch_one(&mut connection)
			.await
			.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(person.id, row.id);
		assert_eq!(person.name, row.name);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test; `SET SCHEMA 'pg_temp';`
	}
}
