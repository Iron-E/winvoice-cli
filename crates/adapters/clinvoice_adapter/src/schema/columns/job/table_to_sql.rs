use super::JobColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for JobColumns<T>
{
	fn default_alias() -> char
	{
		'J'
	}

	fn table_name() -> &'static str
	{
		"jobs"
	}
}
