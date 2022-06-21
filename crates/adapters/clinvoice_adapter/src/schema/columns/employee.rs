mod columns_to_sql;

use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EmployeeColumns<T>
{
	pub id: T,
	pub name: T,
	pub status: T,
	pub title: T,
}

impl<T> EmployeeColumns<T>
{
	/// # Summary
	///
	/// Returns an alternation of [`EmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> EmployeeColumns<WithIdentifier<T, TAlias>>
	where
		TAlias: Copy,
	{
		EmployeeColumns {
			id: WithIdentifier(alias, self.id),
			name: WithIdentifier(alias, self.name),
			status: WithIdentifier(alias, self.status),
			title: WithIdentifier(alias, self.title),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`EmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn typecast<TCast>(self, cast: TCast) -> EmployeeColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		EmployeeColumns {
			id: TypeCast(self.id, cast),
			name: TypeCast(self.name, cast),
			status: TypeCast(self.status, cast),
			title: TypeCast(self.title, cast),
		}
	}
}

impl EmployeeColumns<&'static str>
{
	pub const fn default() -> Self
	{
		Self {
			id: "id",
			name: "name",
			status: "status",
			title: "title",
		}
	}

	pub const fn unique() -> Self
	{
		Self {
			id: "unique_2_employee_id",
			name: "unique_2_employee_name",
			status: "unique_2_employee_status",
			title: "unique_2_employee_title",
		}
	}
}
