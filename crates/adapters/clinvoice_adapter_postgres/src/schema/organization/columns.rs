use clinvoice_schema::{Contact, Location, Organization};
use sqlx::{postgres::PgRow, Row};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgOrganizationColumns<'col>
{
	pub id: &'col str,
	pub location_id: &'col str,
	pub name: &'col str,
}

impl PgOrganizationColumns<'_>
{
	pub(in crate::schema) fn row_to_view(
		self,
		contact_info: Vec<Contact>,
		location: Location,
		row: &PgRow,
	) -> Organization
	{
		Organization {
			contact_info,
			id: row.get(self.id),
			location,
			name: row.get(self.name),
		}
	}
}
