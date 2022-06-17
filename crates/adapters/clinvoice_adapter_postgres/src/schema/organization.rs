use clinvoice_adapter::schema::columns::OrganizationColumns;
use clinvoice_schema::{Location, Organization};
use sqlx::{postgres::PgRow, Row};

mod deletable;
mod organization_adapter;
mod updatable;

pub struct PgOrganization;

impl PgOrganization
{
	pub(super) fn row_to_view(
		columns: OrganizationColumns<&str>,
		row: &PgRow,
		location: Location,
	) -> Organization
	{
		Organization {
			id: row.get(columns.id),
			location,
			name: row.get(columns.name),
		}
	}
}
