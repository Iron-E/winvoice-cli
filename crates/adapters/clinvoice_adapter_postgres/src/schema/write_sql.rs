use clinvoice_adapter::WriteSql;
use super::PostgresSchema;
use clinvoice_query::{Match, MatchStr};

impl WriteSql<Match<'_, i64>> for PostgresSchema
{
	fn write_where(column: &'static str, prefix: Option<&'static str>, query: &Match<'_, i64>, sql: &mut String)
	{
		todo!()
	}
}
