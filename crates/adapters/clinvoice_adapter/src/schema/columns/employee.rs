use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EmployeeColumns<T>
{
	pub id: T,
	pub name: T,
	pub organization_id: T,
	pub status: T,
	pub title: T,
}

impl<T> EmployeeColumns<T>
{
	/// # Summary
	///
	/// Returns an alternation of [`EmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn scoped<TIdent>(self, ident: TIdent) -> EmployeeColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		EmployeeColumns {
			id: WithIdentifier(ident, self.id),
			name: WithIdentifier(ident, self.name),
			organization_id: WithIdentifier(ident, self.organization_id),
			status: WithIdentifier(ident, self.status),
			title: WithIdentifier(ident, self.title),
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
			organization_id: TypeCast(self.organization_id, cast),
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
			organization_id: "organization_id",
			status: "status",
			title: "title",
		}
	}
}
