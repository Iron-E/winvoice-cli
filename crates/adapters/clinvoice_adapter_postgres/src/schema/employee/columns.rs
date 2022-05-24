use clinvoice_schema::{Contact, Employee};
use sqlx::{postgres::PgRow, PgPool, Result, Row};

use crate::schema::organization::columns::PgOrganizationColumns;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgEmployeeColumns<'col>
{
	pub id: &'col str,
	pub name: &'col str,
	pub organization: PgOrganizationColumns<'col>,
	pub status: &'col str,
	pub title: &'col str,
}

impl PgEmployeeColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: &PgPool,
		contact_info: Vec<Contact>,
		row: &PgRow,
	) -> Result<Employee>
	{
		let organization = self.organization.row_to_view(connection, row);

		Ok(Employee {
			contact_info,
			id: row.get(self.id),
			name: row.get(self.name),
			status: row.get(self.status),
			title: row.get(self.title),
			organization: organization.await?,
		})
	}
}
