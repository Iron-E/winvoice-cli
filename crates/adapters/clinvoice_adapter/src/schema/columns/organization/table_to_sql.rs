use super::OrganizationColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for OrganizationColumns<T>
{
	fn table_alias() -> char
	{
		'O'
	}

	fn table_name() -> &'static str
	{
		"organizations"
	}
}
