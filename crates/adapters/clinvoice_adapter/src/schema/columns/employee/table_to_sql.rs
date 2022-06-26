use super::EmployeeColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for EmployeeColumns<T>
{
	fn default_alias() -> char
	{
		'E'
	}

	fn table_name() -> &'static str
	{
		"employees"
	}
}
