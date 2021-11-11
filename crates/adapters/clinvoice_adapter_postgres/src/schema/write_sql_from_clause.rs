use std::fmt::Write;

use clinvoice_adapter::WriteSqlFromClause;

use super::PostgresSchema;

impl WriteSqlFromClause for PostgresSchema
{
	fn write_sql_from_clause(sql: &mut String, table: &'static str, alias: Option<char>)
	{
		write!(sql, " FROM {} {}", table, alias.unwrap_or_default()).unwrap()
	}
}
