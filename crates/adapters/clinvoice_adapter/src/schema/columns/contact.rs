mod columns_to_sql;

use crate::fmt::{As, TypeCast, WithIdentifier};

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
	/// Returns a [`ContactColumns`] which outputs all of its columns as
	/// `column_1 AS aliased_column_1`.
	pub fn r#as<TAlias>(self, aliased: ContactColumns<TAlias>) -> ContactColumns<As<TAlias, T>>
	{
		ContactColumns {
			address_id: As(self.address_id, aliased.address_id),
			email: As(self.email, aliased.email),
			label: As(self.label, aliased.label),
			other: As(self.other, aliased.other),
			phone: As(self.phone, aliased.phone),
		}
	}

	/// # Summary
	///
	/// Returns a [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> ContactColumns<WithIdentifier<T, TAlias>>
	where
		TAlias: Copy,
	{
		ContactColumns {
			address_id: WithIdentifier(alias, self.address_id),
			email: WithIdentifier(alias, self.email),
			label: WithIdentifier(alias, self.label),
			other: WithIdentifier(alias, self.other),
			phone: WithIdentifier(alias, self.phone),
		}
	}

	/// # Summary
	///
	/// Returns a [`ContactColumns`] which modifies its fields' [`Display`]
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
