use super::OrganizationColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for OrganizationColumns<T>
{
	fn default_alias() -> char
	{
		'O'
	}

	fn table_name() -> &'static str
	{
		"organizations"
	}
}
