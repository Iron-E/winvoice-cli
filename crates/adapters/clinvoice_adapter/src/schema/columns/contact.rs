mod columns_to_sql;

use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContactColumns<T>
{
	pub address_id: T,
	pub email: T,
	pub label: T,
	pub phone: T,
	pub username: T,
	pub wallet: T,
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
			phone: WithIdentifier(ident, self.phone),
			username: WithIdentifier(ident, self.username),
			wallet: WithIdentifier(ident, self.wallet),
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
			phone: TypeCast(self.phone, cast),
			username: TypeCast(self.username, cast),
			wallet: TypeCast(self.wallet, cast),
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
			phone: "phone",
			username: "username",
			wallet: "wallet",
		}
	}
}
