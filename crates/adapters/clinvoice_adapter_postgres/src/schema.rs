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

use clinvoice_adapter::{
	fmt::{sql, As, ColumnsToSql, QueryBuilderExt, SnakeCase, TableToSql},
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
	async fn delete<'args, TConn, TIter, TTable>(connection: TConn, entities: TIter) -> Result<()>
	where
		TConn: Executor<'args, Database = Postgres>,
		TIter: Iterator<Item = Id>,
		TTable: TableToSql,
	{
		let mut peekable_entities = entities.peekable();

		// There is nothing to do
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let mut query = QueryBuilder::new(sql::DELETE_FROM);
		query.push(TTable::table_name());

		PgSchema::write_where_clause(
			Default::default(),
			"id",
			&Match::Or(peekable_entities.map(Match::from).collect()),
			&mut query,
		);

		query.prepare().execute(connection).await?;

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
		push_values: impl FnOnce(&mut QueryBuilder<'args, Postgres>),
	) -> Result<()>
	where
		C: ColumnsToSql,
	{
		let alias = C::default_alias();
		let mut query = QueryBuilder::new(sql::UPDATE);

		query.push(As(C::table_name(), alias)).push(sql::SET);

		let values_alias = SnakeCase::from((alias, "V"));
		columns.push_set_to(&mut query, values_alias);

		query.push(sql::FROM).push('(');

		push_values(&mut query);

		query
			.push(')')
			.push(sql::AS)
			.push(values_alias)
			.push(" (")
			.push_columns(&columns)
			.push(')')
			.push(sql::WHERE);

		columns.push_update_where_to(&mut query, alias, values_alias);

		query.prepare().execute(connection).await?;

		Ok(())
	}
}
