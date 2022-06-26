use super::LocationColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for LocationColumns<T>
{
	fn default_alias() -> char
	{
		'L'
	}

	fn table_name() -> &'static str
	{
		"locations"
	}
}
