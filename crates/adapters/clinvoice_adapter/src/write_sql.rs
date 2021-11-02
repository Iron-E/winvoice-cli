pub trait WriteSql<Q>
{
	fn write_sql(column: &'static str, prefix: Option<&'static str>, query: &Q, sql: &mut String);
}
