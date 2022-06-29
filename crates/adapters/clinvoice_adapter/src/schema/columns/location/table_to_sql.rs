use super::LocationColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for LocationColumns<T>
{
	const DEFAULT_ALIAS: char = 'L';
	const TABLE_NAME: &'static str = "locations";
}
