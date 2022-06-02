use core::fmt::Display;

use clinvoice_schema::{Contact, Location, Organization};
use sqlx::{postgres::PgRow, Row};

use crate::schema::PgScopedColumn;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgOrganizationColumns<D>
where
	D: Display,
{
	pub id: D,
	pub location_id: D,
	pub name: D,
}

impl<D> PgOrganizationColumns<D>
where
	D: Copy + Display,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgOrganizationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgOrganizationColumns<PgScopedColumn<D, TIdent>>
	where
		TIdent: Copy + Display,
	{
		PgOrganizationColumns {
			id: PgScopedColumn(ident, self.id),
			location_id: PgScopedColumn(ident, self.location_id),
			name: PgScopedColumn(ident, self.name),
		}
	}
}

impl PgOrganizationColumns<&str>
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

impl PgOrganizationColumns<&'static str>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			id: "id",
			location_id: "location_id",
			name: "name",
		}
	}
}
