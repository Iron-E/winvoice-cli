use super::EmployeeColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for EmployeeColumns<T>
{
	const DEFAULT_ALIAS: char = 'E';
	const TABLE_NAME: &'static str = "employees";
}
