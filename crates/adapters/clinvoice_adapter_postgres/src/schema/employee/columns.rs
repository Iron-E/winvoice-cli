use clinvoice_schema::{Employee, Organization};
use sqlx::{postgres::PgRow, Row};

use crate::schema::{typecast::PgTypeCast, PgScopedColumn};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgEmployeeColumns<T>
{
	pub id: T,
	pub name: T,
	pub organization_id: T,
	pub status: T,
	pub title: T,
}

impl<T> PgEmployeeColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgEmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgEmployeeColumns<PgScopedColumn<T, TIdent>>
	where
		TIdent: Copy,
	{
		PgEmployeeColumns {
			id: PgScopedColumn(ident, self.id),
			name: PgScopedColumn(ident, self.name),
			organization_id: PgScopedColumn(ident, self.organization_id),
			status: PgScopedColumn(ident, self.status),
			title: PgScopedColumn(ident, self.title),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`PgEmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn typecast<TCast>(
		&self,
		cast: TCast,
	) -> PgEmployeeColumns<PgTypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		PgEmployeeColumns {
			id: PgTypeCast(self.id, cast),
			name: PgTypeCast(self.name, cast),
			organization_id: PgTypeCast(self.organization_id, cast),
			status: PgTypeCast(self.status, cast),
			title: PgTypeCast(self.title, cast),
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
