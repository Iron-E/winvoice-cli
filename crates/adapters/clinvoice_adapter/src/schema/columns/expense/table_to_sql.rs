use super::ExpenseColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for ExpenseColumns<T>
{
	const DEFAULT_ALIAS: char = 'X';
	const TABLE_NAME: &'static str = "expenses";
}
