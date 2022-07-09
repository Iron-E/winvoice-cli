mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

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
	/// Returns a [`ContactColumns`] which aliases the names of these [`ContactColumns`] with the
	/// `aliased` columns provided.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::schema::columns::ContactColumns;
	///
	/// assert_eq!(
	///   ContactColumns::default()
	///     .default_scope()
	///     .r#as(ContactColumns {
	///       address_id: "one",
	///       email: "two",
	///       label: "three",
	///       other: "four",
	///       phone: "five",
	///     })
	///     .address_id
	///     .to_string(),
	///   "C.address_id AS one",
	/// );
	/// ```
	pub fn r#as<TAlias>(self, aliased: ContactColumns<TAlias>) -> ContactColumns<As<T, TAlias>>
	{
		ContactColumns {
			address_id: As(self.address_id, aliased.address_id),
			email: As(self.email, aliased.email),
			label: As(self.label, aliased.label),
			other: As(self.other, aliased.other),
			phone: As(self.phone, aliased.phone),
		}
	}

	/// Add a [scope](ContactColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # Examples
	///
	/// * See [`ContactColumns::r#as`].
	pub fn default_scope(self) -> ContactColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// Returns a [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # Examples
	///
	/// * See [`ContactColumns::default_scope`].
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

	/// Returns a [`ContactColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::schema::columns::ContactColumns;
	///
	/// assert_eq!(
	///   ContactColumns::default().typecast("text").address_id.to_string(),
	///   " CAST (address_id AS text)",
	/// );
	/// ```
	pub fn typecast<TCast>(self, cast: TCast) -> ContactColumns<TypeCast<T, TCast>>
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
	/// The names of the columns in `contact_information` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`ContactColumns::r#as`].
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
