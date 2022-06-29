use super::JobColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for JobColumns<T>
{
	const DEFAULT_ALIAS: char = 'J';
	const TABLE_NAME: &'static str = "jobs";
}
