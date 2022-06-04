use clinvoice_adapter::schema::columns::OrganizationColumns;
use clinvoice_schema::{Contact, Location, Organization};
use sqlx::{postgres::PgRow, Row};

mod deletable;
mod organization_adapter;
mod updatable;

pub struct PgOrganization;

impl PgOrganization
{
	pub(in crate::schema) fn row_to_view(
		columns: OrganizationColumns<&str>,
		row: &PgRow,
		contact_info: Vec<Contact>,
		location: Location,
	) -> Organization
	{
		Organization {
			contact_info,
			id: row.get(columns.id),
			location,
			name: row.get(columns.name),
		}
	}
}
