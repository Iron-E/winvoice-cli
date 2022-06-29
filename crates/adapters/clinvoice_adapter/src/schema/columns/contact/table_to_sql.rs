use super::ContactColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for ContactColumns<T>
{
	const DEFAULT_ALIAS: char = 'C';
	const TABLE_NAME: &'static str = "contact_information";
}
