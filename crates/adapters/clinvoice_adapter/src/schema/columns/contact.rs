use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContactColumns<T>
{
	pub address_id: T,
	pub email: T,
	pub export: T,
	pub label: T,
	pub organization_id: T,
	pub phone: T,
}

impl<T> ContactColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scoped<TIdent>(&self, ident: TIdent) -> ContactColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		ContactColumns {
			address_id: WithIdentifier(ident, self.address_id),
			email: WithIdentifier(ident, self.email),
			export: WithIdentifier(ident, self.export),
			label: WithIdentifier(ident, self.label),
			organization_id: WithIdentifier(ident, self.organization_id),
			phone: WithIdentifier(ident, self.phone),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(&self, cast: TCast) -> ContactColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		ContactColumns {
			address_id: TypeCast(self.address_id, cast),
			email: TypeCast(self.email, cast),
			export: TypeCast(self.export, cast),
			label: TypeCast(self.label, cast),
			organization_id: TypeCast(self.organization_id, cast),
			phone: TypeCast(self.phone, cast),
		}
	}
}

impl ContactColumns<&'static str>
{
	pub const fn default() -> Self
	{
		Self {
			address_id: "address_id",
			email: "email",
			export: "export",
			label: "label",
			organization_id: "organization_id",
			phone: "phone",
		}
	}
}
