mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocationColumns<T>
{
	pub id: T,
	pub name: T,
	pub outer_id: T,
}

impl<T> LocationColumns<T>
{
	/// # Summary
	///
	/// Returns a [`LocationColumns`] which outputs all of its columns as
	/// `column_1 AS aliased_column_1`.
	pub fn r#as<TAlias>(self, aliased: LocationColumns<TAlias>) -> LocationColumns<As<T, TAlias>>
	{
		LocationColumns {
			id: As(self.id, aliased.id),
			name: As(self.name, aliased.name),
			outer_id: As(self.outer_id, aliased.outer_id),
		}
	}

	/// # Summary
	///
	/// Add a [scope](Self::scope) using the [default alias](TableToSql::default_alias)
	pub fn default_scope(self) -> LocationColumns<WithIdentifier<T, char>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// # Summary
	///
	/// Returns a [`LocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> LocationColumns<WithIdentifier<T, TAlias>>
	where
		TAlias: Copy,
	{
		LocationColumns {
			id: WithIdentifier(alias, self.id),
			outer_id: WithIdentifier(alias, self.outer_id),
			name: WithIdentifier(alias, self.name),
		}
	}

	/// # Summary
	///
	/// Returns a [`LocationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> LocationColumns<TypeCast<T, TCast>>
	where
		TCast: Copy,
	{
		LocationColumns {
			id: TypeCast(self.id, cast),
			outer_id: TypeCast(self.outer_id, cast),
			name: TypeCast(self.name, cast),
		}
	}
}

impl LocationColumns<&'static str>
{
	pub const fn default() -> Self
	{
		Self {
			id: "id",
			outer_id: "outer_id",
			name: "name",
		}
	}

	pub const fn unique() -> Self
	{
		Self {
			id: "unique_5_location_id",
			outer_id: "unique_5_location_outer_id",
			name: "unique_5_location_name",
		}
	}
}
