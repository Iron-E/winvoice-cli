use clinvoice_schema::views::OrganizationView;
use sqlx::{postgres::PgRow, Executor, Postgres, Result, Row};

use crate::schema::PgLocation;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgOrganizationColumns<'col>
{
	pub id: &'col str,
	pub location_id: &'col str,
	pub name: &'col str,
}

impl PgOrganizationColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: impl Executor<'_, Database = Postgres>,
		row: &PgRow,
	) -> Result<OrganizationView>
	{
		Ok(OrganizationView {
			id: row.get(self.id),
			location: PgLocation::retrieve_view_by_id(connection, row.get(self.location_id)).await?,
			name: row.get(self.name),
		})
	}
}
