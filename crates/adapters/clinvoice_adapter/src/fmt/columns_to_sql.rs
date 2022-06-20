use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

/// # Summary
///
/// This trait defines methods which are commonly used when generating SQL that references all of
/// the columns for a given table.
pub trait ColumnsToSql
{
	/// # Summary
	///
	/// Push a comma-separated list of column names to the `query`, e.g.: `column_1,column_2,`…`column_n`.
	fn push<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database;

	/// # Summary
	///
	/// Push the `SET` clause (keyword not included) to the `query`, e.g.:
	///
	/// ```sql
	/// column_1 = {values_ident}.column_1,
	/// column_2 = {values_ident}.column_2,
	/// …column_n = {values_ident}.column_n
	/// ```
	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database;

	/// # Summary
	///
	/// Push the `WHERE` clause of an `UPDATE` statement (`WHERE` keyword not included) to the `query`, e.g.:
	///
	/// ```sql
	/// id = {values_ident}.id
	/// ```
	fn push_update_where<Db>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_ident: impl Copy + Display,
		values_ident: impl Copy + Display,
	) where
		Db: Database;
}
