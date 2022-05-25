use clinvoice_schema::{Employee, Organization};
use sqlx::{postgres::PgRow, Result, Row};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgEmployeeColumns<'col>
{
	pub id: &'col str,
	pub name: &'col str,
	pub organization_id: &'col str,
	pub status: &'col str,
	pub title: &'col str,
}

impl PgEmployeeColumns<'_>
{
	pub(in crate::schema) fn row_to_view(
		self,
		organization: Organization,
		row: &PgRow,
	) -> Result<Employee>
	{
		Ok(Employee {
			id: row.get(self.id),
			name: row.get(self.name),
			organization,
			status: row.get(self.status),
			title: row.get(self.title),
		})
	}
}
