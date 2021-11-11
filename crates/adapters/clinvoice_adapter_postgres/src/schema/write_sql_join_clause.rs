use std::fmt::Write;

use clinvoice_adapter::WriteSqlJoinClause;

use super::PostgresSchema;

impl WriteSqlJoinClause for PostgresSchema
{
	fn write_sql_join_clause(
		sql: &mut String,
		join_table: &'static str,
		join_alias: char,
		join_column: &'static str,
		base_column: &'static str,
	)
	{
		write!(
			sql,
			" JOIN {} {alias} ON ({alias}.{} = {})",
			join_table,
			join_column,
			base_column,
			alias = join_alias
		)
		.unwrap()
	}
}
