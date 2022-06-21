//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::Deletable)) for a Postgres filesystem.

mod contact_info;
mod employee;
mod expenses;
mod initializable;
mod job;
mod location;
mod organization;
mod timesheet;
mod util;
mod write_where_clause;

use core::fmt::Display;

use clinvoice_adapter::{
	fmt::{As, ColumnsToSql, SnakeCase},
	WriteWhereClause,
};
use clinvoice_match::Match;
use clinvoice_schema::Id;
pub use contact_info::PgContactInfo;
pub use employee::PgEmployee;
pub use expenses::PgExpenses;
pub use job::PgJob;
pub use location::PgLocation;
pub use organization::PgOrganization;
use sqlx::{Executor, Postgres, QueryBuilder, Result, Transaction};
pub use timesheet::PgTimesheet;

/// # Summary
///
/// An empty struct which implements [`Initializable`](clinvoice_adapter::schema::Initializable) so
/// that the Postgres database can have all of the necessary tables set up if this is the first run
/// of the program.
pub struct PgSchema;

impl PgSchema
{
	/// # Summary
	///
	/// Execute `DELETE FROM {table} WHERE (id = №) OR … OR (id = №)`
	/// for each [`Id`] in `entities.
	async fn delete(
		connection: impl Executor<'_, Database = Postgres>,
		table: impl Display,
		entities: impl Iterator<Item = Id>,
	) -> Result<()>
	{
		let mut peekable_entities = entities.peekable();

		// There is nothing to do
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let mut query = QueryBuilder::new("DELETE FROM ");
		query.push(table);

		PgSchema::write_where_clause(
			Default::default(),
			"id",
			&Match::Or(peekable_entities.map(Match::from).collect()),
			&mut query,
		);

		query.push(';').build().execute(connection).await?;

		Ok(())
	}

	/// # Summary
	///
	/// Execute a query over the given `connection` which updates `columns` of a `table` given
	/// the some values specified by `push_values` (e.g.
	/// `|query| query.push_values(my_iterator, |mut q, value| …)`).
	///
	/// # See
	///
	/// * [`ColumnsToSql::push_columns`] for how the order of columns to bind in `push_values`.
	/// * [`ColumnsToSql::push_set`] for how the `SET` clause is generated.
	/// * [`ColumnsToSql::push_update_where`] for how the `WHERE` condition is generated.
	/// * [`QueryBuilder::push_values`] for what function to use for `push_values`.
	async fn update<'args, C>(
		connection: &mut Transaction<'_, Postgres>,
		columns: C,
		table: impl Display,
		table_alias: impl Copy + Display,
		push_values: impl FnOnce(&mut QueryBuilder<'args, Postgres>),
	) -> Result<()>
	where
		C: ColumnsToSql,
	{
		let mut query = QueryBuilder::new("");
		query
			.separated(' ')
			.push("UPDATE")
			.push(As(table, table_alias))
			.push("SET ");

		let values_alias = SnakeCase::from((table_alias, "V"));
		columns.push_set(&mut query, values_alias);

		query.push(" FROM (");

		push_values(&mut query);

		query
			.separated(' ')
			.push(") AS")
			.push(values_alias)
			.push('(');

		columns.push(&mut query);

		query.push(") WHERE ");

		columns.push_update_where(&mut query, table_alias, values_alias);

		query.push(';').build().execute(connection).await?;

		Ok(())
	}
}
