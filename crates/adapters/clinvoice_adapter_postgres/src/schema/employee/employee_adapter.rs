use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, TableToSql},
	schema::{columns::EmployeeColumns, EmployeeAdapter},
	WriteWhereClause,
};
use clinvoice_match::MatchEmployee;
use clinvoice_schema::Employee;
use futures::TryStreamExt;
use sqlx::{PgPool, QueryBuilder, Result};

use super::PgEmployee;
use crate::PgSchema;

#[async_trait::async_trait]
impl EmployeeAdapter for PgEmployee
{
	async fn create(
		connection: &PgPool,
		name: String,
		status: String,
		title: String,
	) -> Result<Employee>
	{
		let row = sqlx::query!(
			"INSERT INTO employees (name, status, title) VALUES ($1, $2, $3) RETURNING id;",
			name,
			status,
			title,
		)
		.fetch_one(connection)
		.await?;

		Ok(Employee {
			id: row.id,
			name,
			status,
			title,
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: &MatchEmployee)
		-> Result<Vec<Employee>>
	{
		const COLUMNS: EmployeeColumns<&'static str> = EmployeeColumns::default();

		let mut query = QueryBuilder::new(sql::SELECT);

		query
			.push_columns(&COLUMNS.default_scope())
			.push_default_from::<EmployeeColumns<char>>();

		PgSchema::write_where_clause(
			Default::default(),
			EmployeeColumns::<char>::DEFAULT_ALIAS,
			match_condition,
			&mut query,
		);

		query
			.prepare()
			.fetch(connection)
			.map_ok(|row| PgEmployee::row_to_view(COLUMNS, &row))
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashSet;

	use clinvoice_match::{Match, MatchEmployee, MatchStr};
	use pretty_assertions::assert_eq;

	use super::{EmployeeAdapter, PgEmployee};
	use crate::schema::util;

	#[tokio::test]
	async fn create()
	{
		let connection = util::connect().await;

		let employee = PgEmployee::create(
			&connection,
			"My Name".into(),
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!("SELECT * FROM employees WHERE id = $1;", employee.id)
			.fetch_one(&connection)
			.await
			.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(employee.id, row.id);
		assert_eq!(employee.name, row.name);
		assert_eq!(employee.status, row.status);
		assert_eq!(employee.title, row.title);
	}

	#[tokio::test]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				"My Name".into(),
				"Employed".into(),
				"Janitor".into(),
			),
			PgEmployee::create(
				&connection,
				"Another Gúy".into(),
				"Management".into(),
				"Assistant to Regional Manager".into(),
			),
		)
		.unwrap();

		assert_eq!(
			PgEmployee::retrieve(&connection, &MatchEmployee {
				id: Match::Or(vec![employee.id.into(), employee2.id.into()]),
				name: employee.name.clone().into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[employee.clone()],
		);

		assert_eq!(
			PgEmployee::retrieve(&connection, &MatchEmployee {
				id: Match::Or(vec![employee.id.into(), employee2.id.into()]),
				name: MatchStr::Not(MatchStr::from("Fired".to_string()).into()),
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>(),
			[employee, employee2].into_iter().collect()
		);
	}
}
