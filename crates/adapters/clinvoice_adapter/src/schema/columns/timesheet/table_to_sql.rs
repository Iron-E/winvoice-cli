use super::TimesheetColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for TimesheetColumns<T>
{
	fn default_alias() -> char
	{
		'T'
	}

	fn table_name() -> &'static str
	{
		"timesheets"
	}
}
