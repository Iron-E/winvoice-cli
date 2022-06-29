mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganizationColumns<T>
{
	pub id: T,
	pub location_id: T,
	pub name: T,
}

impl<T> OrganizationColumns<T>
{
	/// # Summary
	///
	/// Returns a [`OrganizationColumns`] which outputs all of its columns as
	/// `column_1 AS aliased_column_1`.
	pub fn r#as<TAlias>(
		self,
		aliased: OrganizationColumns<TAlias>,
	) -> OrganizationColumns<As<TAlias, T>>
	{
		OrganizationColumns {
			id: As(self.id, aliased.id),
			location_id: As(self.location_id, aliased.location_id),
			name: As(self.name, aliased.name),
		}
	}

	/// # Summary
	///
	/// Add a [scope](Self::scope) using the [default alias](TableToSql::default_alias)
	pub fn default_scope(self) -> OrganizationColumns<WithIdentifier<T, char>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// # Summary
	///
	/// Returns a [`OrganizationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> OrganizationColumns<WithIdentifier<T, TAlias>>
	where
		TAlias: Copy,
	{
		OrganizationColumns {
			id: WithIdentifier(alias, self.id),
			location_id: WithIdentifier(alias, self.location_id),
			name: WithIdentifier(alias, self.name),
		}
	}

	/// # Summary
	///
	/// Returns a [`OrganizationColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> OrganizationColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		OrganizationColumns {
			id: TypeCast(self.id, cast),
			location_id: TypeCast(self.location_id, cast),
			name: TypeCast(self.name, cast),
		}
	}
}

impl OrganizationColumns<&'static str>
{
	pub const fn default() -> Self
	{
		Self {
			id: "id",
			location_id: "location_id",
			name: "name",
		}
	}

	pub const fn unique() -> Self
	{
		Self {
			id: "unique_6_organization_id",
			location_id: "unique_6_organization_location_id",
			name: "unique_6_organization_name",
		}
	}
}
