use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::TableToSql;

/// # Summary
///
/// This trait defines methods which are commonly used when generating SQL that references all of
/// the columns for a given table.
pub trait ColumnsToSql: TableToSql
{
	/// # Summary
	///
	/// Push a comma-separated list of column names to the `query`, e.g.: `column_1,column_2,`…`column_n`.
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database;

	/// # Summary
	///
	/// Push the `SET` clause (keyword not included) to the `query`, e.g.:
	///
	/// ```sql
	/// column_1 = values_alias.column_1,
	/// column_2 = values_alias.column_2,
	/// …column_n = values_alias.column_n
	/// ```
	fn push_set_to<Db>(&self, query: &mut QueryBuilder<Db>, values_alias: impl Copy + Display)
	where
		Db: Database;

	/// # Summary
	///
	/// Push the `WHERE` clause of an `UPDATE` statement (`WHERE` keyword not included) to the `query`, e.g.:
	///
	/// ```sql
	/// table_alias.id = values_alias.id
	/// ```
	fn push_update_where_to<Db>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_alias: impl Copy + Display,
		values_alias: impl Copy + Display,
	) where
		Db: Database;
}
