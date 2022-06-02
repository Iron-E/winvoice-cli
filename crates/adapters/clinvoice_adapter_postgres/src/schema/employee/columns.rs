use core::fmt::Display;

use clinvoice_schema::{Employee, Organization};
use sqlx::{postgres::PgRow, Row};

use crate::schema::PgScopedColumn;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgEmployeeColumns<D>
where
	D: Display,
{
	pub id: D,
	pub name: D,
	pub organization_id: D,
	pub status: D,
	pub title: D,
}

impl<D> PgEmployeeColumns<D>
where
	D: Copy + Display,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgEmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgEmployeeColumns<PgScopedColumn<D, TIdent>>
	where
		TIdent: Copy + Display,
	{
		PgEmployeeColumns {
			id: PgScopedColumn(ident, self.id),
			name: PgScopedColumn(ident, self.name),
			organization_id: PgScopedColumn(ident, self.organization_id),
			status: PgScopedColumn(ident, self.status),
			title: PgScopedColumn(ident, self.title),
		}
	}
}

impl PgEmployeeColumns<&str>
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

impl PgEmployeeColumns<&'static str>
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
