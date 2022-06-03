use clinvoice_schema::{Contact, Location, Organization};
use sqlx::{postgres::PgRow, Row};

use crate::schema::{typecast::PgTypeCast, PgScopedColumn};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgOrganizationColumns<T>
{
	pub id: T,
	pub location_id: T,
	pub name: T,
}

impl<T> PgOrganizationColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgOrganizationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgOrganizationColumns<PgScopedColumn<T, TIdent>>
	where
		TIdent: Copy,
	{
		PgOrganizationColumns {
			id: PgScopedColumn(ident, self.id),
			location_id: PgScopedColumn(ident, self.location_id),
			name: PgScopedColumn(ident, self.name),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`PgOrganizationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub(in crate::schema) fn typecast<TCast>(
		&self,
		cast: TCast,
	) -> PgOrganizationColumns<PgTypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		PgOrganizationColumns {
			id: PgTypeCast(self.id, cast),
			location_id: PgTypeCast(self.location_id, cast),
			name: PgTypeCast(self.name, cast),
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
