use core::fmt::Display;

use crate::schema::{PgScopedColumn, typecast::PgTypeCast};

pub(in crate::schema) struct PgLocationColumns<D>
where
	D: Display,
{
	pub id: D,
	pub outer_id: D,
	pub name: D,
}

impl<D> PgLocationColumns<D>
where
	D: Copy + Display,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgLocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgLocationColumns<PgScopedColumn<D, TIdent>>
	where
		TIdent: Copy + Display,
	{
		PgLocationColumns {
			id: PgScopedColumn(ident, self.id),
			outer_id: PgScopedColumn(ident, self.outer_id),
			name: PgScopedColumn(ident, self.name),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`PgLocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub(in crate::schema) fn typecast<TCast>(
		&self,
		cast: TCast,
	) -> PgLocationColumns<PgTypeCast<TCast, D>>
	where
		TCast: Display,
	{
		PgLocationColumns {
			id: PgTypeCast(self.id, cast),
			outer_id: PgTypeCast(self.outer_id, cast),
			name: PgTypeCast(self.name, cast),
		}
	}
}

impl PgLocationColumns<&'static str>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			id: "id",
			outer_id: "outer_id",
			name: "name",
		}
	}
}
