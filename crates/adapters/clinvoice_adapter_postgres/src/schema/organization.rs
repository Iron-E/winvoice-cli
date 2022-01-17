use clinvoice_schema::views::OrganizationView;
use sqlx::{postgres::PgRow, Executor, Postgres, Result, Row};

use super::PgLocation;

mod deletable;
mod organization_adapter;
mod updatable;

pub struct PgOrganization;

impl PgOrganization
{
	pub(super) async fn row_to_view(
		row: &PgRow,
		connection: impl Executor<'_, Database = Postgres>,
		id: &str,
		location_id: &str,
		name: &str,
	) -> Result<OrganizationView>
	{
		Ok(OrganizationView {
			id: row.get(id),
			location: PgLocation::retrieve_view_by_id(connection, row.get(location_id)).await?,
			name: row.get(name),
		})
	}
}
