use super::LocationColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for LocationColumns<T>
{
	fn table_alias() -> char
	{
		'L'
	}

	fn table_name() -> &'static str
	{
		"locations"
	}
}
