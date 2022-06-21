mod columns_to_sql;

use crate::fmt::{TypeCast, WithIdentifier};

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
	/// Returns an alternation of [`OrganizationColumns`] which modifies its fields' [`Display`]
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
	/// Returns an alternation of [`OrganizationColumns`] which modifies its fields' [`Display`]
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
