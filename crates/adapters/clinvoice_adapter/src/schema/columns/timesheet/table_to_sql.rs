use super::TimesheetColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for TimesheetColumns<T>
{
	const DEFAULT_ALIAS: char = 'T';
	const TABLE_NAME: &'static str = "timesheets";
}
