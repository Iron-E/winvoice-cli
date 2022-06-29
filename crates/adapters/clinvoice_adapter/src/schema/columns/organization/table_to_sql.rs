use super::OrganizationColumns;
use crate::fmt::TableToSql;

impl<T> TableToSql for OrganizationColumns<T>
{
	const DEFAULT_ALIAS: char = 'O';
	const TABLE_NAME: &'static str = "organizations";
}
