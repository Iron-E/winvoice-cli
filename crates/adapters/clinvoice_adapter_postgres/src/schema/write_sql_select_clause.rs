use clinvoice_adapter::WriteSqlSelectClause;

use super::PostgresSchema;

impl WriteSqlSelectClause for PostgresSchema
{
	fn write_sql_select_clause<const N: usize>(columns: [&'static str; N]) -> String
	{
		format!("SELECT {}", columns.join(","))
	}
}
