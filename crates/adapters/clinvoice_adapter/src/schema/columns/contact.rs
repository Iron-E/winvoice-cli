mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{TableToSql, WithIdentifier};

/// The names of the columns of the `contact_information` table.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContactColumns<T>
{
	/// The name of the `address_id` column of the `contact_information` table.
	pub address_id: T,

	/// The name of the `email` column of the `contact_information` table.
	pub email: T,

	/// The name of the `label` column of the `contact_information` table.
	pub label: T,

	/// The name of the `other` column of the `contact_information` table.
	pub other: T,

	/// The name of the `phone` column of the `contact_information` table.
	pub phone: T,
}

impl<T> ContactColumns<T>
{
	/// Add a [scope](ContactColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> ContactColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// Returns a [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn scope<TAlias>(self, alias: TAlias) -> ContactColumns<WithIdentifier<TAlias, T>>
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
}

impl ContactColumns<&'static str>
{
	/// The names of the columns in `contact_information` without any aliasing.
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
}
