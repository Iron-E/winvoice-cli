use crate::schema::{typecast::PgTypeCast, PgScopedColumn};

pub(in crate::schema) struct PgLocationColumns<T>
{
	pub id: T,
	pub outer_id: T,
	pub name: T,
}

impl<T> PgLocationColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgLocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgLocationColumns<PgScopedColumn<T, TIdent>>
	where
		TIdent: Copy,
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
	) -> PgLocationColumns<PgTypeCast<TCast, T>>
	where
		TCast: Copy,
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
