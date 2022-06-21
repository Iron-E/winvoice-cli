mod columns_to_sql;

use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContactColumns<T>
{
	pub address_id: T,
	pub email: T,
	pub label: T,
	pub other: T,
	pub phone: T,
}

impl<T> ContactColumns<T>
{
	/// # Summary
	///
	/// Returns an alternation of [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scoped<TIdent>(self, ident: TIdent) -> ContactColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		ContactColumns {
			address_id: WithIdentifier(ident, self.address_id),
			email: WithIdentifier(ident, self.email),
			label: WithIdentifier(ident, self.label),
			other: WithIdentifier(ident, self.other),
			phone: WithIdentifier(ident, self.phone),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> ContactColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		ContactColumns {
			address_id: TypeCast(self.address_id, cast),
			email: TypeCast(self.email, cast),
			label: TypeCast(self.label, cast),
			other: TypeCast(self.other, cast),
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
			label: "label",
			other: "other",
			phone: "phone",
		}
	}

	pub const fn unique() -> Self
	{
		Self {
			address_id: "unique_1_contact_address_id",
			email: "unique_1_contact_email",
			label: "unique_1_contact_label",
			other: "unique_1_contact_other",
			phone: "unique_1_contact_phone",
		}
	}
}
