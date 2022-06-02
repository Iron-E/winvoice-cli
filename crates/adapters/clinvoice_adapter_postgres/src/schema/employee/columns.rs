use clinvoice_schema::{Employee, Organization};
use sqlx::{postgres::PgRow, Row};

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
	pub(in crate::schema) fn row_to_view(self, organization: Organization, row: &PgRow) -> Employee
	{
		Employee {
			id: row.get(self.id),
			name: row.get(self.name),
			organization,
			status: row.get(self.status),
			title: row.get(self.title),
		}
	}
}

impl PgEmployeeColumns<'static>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			id: "id",
			name: "name",
			organization_id: "organization_id",
			status: "status",
			title: "title",
		}
	}
}
