use super::ContactColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for ContactColumns<T>
{
	fn table_alias() -> char
	{
		'C'
	}

	fn table_name() -> &'static str
	{
		"contact_information"
	}
}
