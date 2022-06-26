use super::ExpenseColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for ExpenseColumns<T>
{
	fn default_alias() -> char
	{
		'X'
	}

	fn table_name() -> &'static str
	{
		"expenses"
	}
}
