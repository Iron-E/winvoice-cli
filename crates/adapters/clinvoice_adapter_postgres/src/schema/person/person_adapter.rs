use std::{fmt::Write, ops::Deref};

use clinvoice_adapter::schema::PersonAdapter;
use clinvoice_query::{Person as QueryPerson, Match, MatchStr};
use clinvoice_schema::{views::PersonView, Person};
use futures::stream::TryStreamExt;
use sqlx::{postgres::Postgres, Executor, Result, Row};

use super::PostgresPerson;

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
		query: &QueryPerson,
	) -> Result<Vec<PersonView>>
	{
		let mut sql = String::with_capacity(21);
		write!(sql, "SELECT * FROM people").unwrap();

		if query != &Default::default()
		{
			write!(sql, " WHERE").unwrap();
		}

		if query.id != Match::Any
		{
			fn write_query(prefix: Option<&str>, query: &Match<'_, i64>, sql: &mut String)
			{
				// TODO: make `ToSql` trait in `clinvoice_adapter` with `impl` on `Schema`
				match query
				{
					Match::AllGreaterThan(id) | Match::GreaterThan(id) =>
					{
						write!(sql, " {} id > {}", prefix.unwrap_or_default(), id).unwrap()
					},
					Match::AllLessThan(id) | Match::LessThan(id) =>
					{
						write!(sql, " {} id < {}", prefix.unwrap_or_default(), id).unwrap()
					},
					Match::AllInRange(low, high) | Match::InRange(low, high) => write!(
						sql,
						" {} {} <= id AND id < {}",
						prefix.unwrap_or_default(),
						low,
						high
					)
					.unwrap(),
					Match::And(queries) =>
					{
						prefix.map(|p| write!(sql, " {}", p).unwrap());
						queries.first().map(|q| write_query(Some("("), q, sql));
						queries
							.iter()
							.skip(1)
							.for_each(|q| write_query(Some("AND"), q, sql));
						write!(sql, ")").unwrap();
					},
					Match::EqualTo(id) =>
					{
						write!(sql, " {} id = {}", prefix.unwrap_or_default(), id).unwrap()
					},
					Match::HasAll(ids) =>
					{
						let mut iter = ids.iter();
						iter.next().map(|id| {
							write!(sql, " {} id = ALL(ARRAY[{}", prefix.unwrap_or_default(), id).unwrap()
						});
						iter.for_each(|id| write!(sql, ", {}", id).unwrap());
						write!(sql, "])").unwrap();
					},
					Match::HasAny(ids) =>
					{
						let mut iter = ids.iter();
						iter.next().map(|id| {
							write!(sql, " {} id IN ({}", prefix.unwrap_or_default(), id).unwrap()
						});
						iter.for_each(|id| write!(sql, ", {}", id).unwrap());
						write!(sql, ")").unwrap();
					},
					Match::Not(positive_query) =>
					{
						prefix.map(|p| write!(sql, " {}", p).unwrap());
						write_query(Some("NOT ("), positive_query.deref(), sql);
						write!(sql, ") ").unwrap();
					},
					Match::Or(queries) =>
					{
						prefix.map(|p| write!(sql, " {}", p).unwrap());
						queries.first().map(|q| write_query(Some("("), q, sql));
						queries
							.iter()
							.skip(1)
							.for_each(|q| write_query(Some("OR"), q, sql));
						write!(sql, ")").unwrap();
					},
					_ => unreachable!("`query.id` is not supposed to be `Any` by this point"),
				}
			}
			write_query(None, &query.id, &mut sql);
		};

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
