use clinvoice_adapter::schema::columns::OrganizationColumns;
use clinvoice_schema::Organization;
use sqlx::{postgres::PgRow, Executor, Postgres, Result, Row};

use super::PgLocation;

mod deletable;
mod organization_adapter;
mod updatable;

pub struct PgOrganization;

impl PgOrganization
{
	pub(super) async fn row_to_view(
		connection: impl Executor<'_, Database = Postgres>,
		columns: OrganizationColumns<&str>,
		row: &PgRow,
	) -> Result<Organization>
	{
		let location_id = row.try_get(columns.location_id)?;
		Ok(Organization {
			id: row.try_get(columns.id)?,
			location: PgLocation::retrieve_by_id(connection, location_id).await?,
			name: row.try_get(columns.name)?,
		})
	}
}
