mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{TableToSql, TypeCast, WithIdentifier};

/// The names of the columns of the `expenses` table.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpenseColumns<T>
{
	/// The name of the `category` column of the `expenses` table.
	pub category: T,

	/// The name of the `cost` column of the `expenses` table.
	pub cost: T,

	/// The name of the `description` column of the `expenses` table.
	pub description: T,

	/// The name of the `id` column of the `expenses` table.
	pub id: T,

	/// The name of the `timesheet_id` column of the `expenses` table.
	pub timesheet_id: T,
}

impl<T> ExpenseColumns<T>
{
	/// Add a [scope](ExpenseColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> ExpenseColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// Returns a [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn scope<TAlias>(self, alias: TAlias) -> ExpenseColumns<WithIdentifier<TAlias, T>>
	where
		TAlias: Copy,
	{
		ExpenseColumns {
			id: WithIdentifier(alias, self.id),
			timesheet_id: WithIdentifier(alias, self.timesheet_id),
			category: WithIdentifier(alias, self.category),
			cost: WithIdentifier(alias, self.cost),
			description: WithIdentifier(alias, self.description),
		}
	}

	/// Returns a [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	///
	/// # See also
	///
	/// * [`TypeCast`]
	pub fn typecast<TCast>(self, cast: TCast) -> ExpenseColumns<TypeCast<T, TCast>>
	where
		TCast: Copy,
	{
		ExpenseColumns {
			id: TypeCast(self.id, cast),
			timesheet_id: TypeCast(self.timesheet_id, cast),
			category: TypeCast(self.category, cast),
			cost: TypeCast(self.cost, cast),
			description: TypeCast(self.description, cast),
		}
	}
}

impl ExpenseColumns<&'static str>
{
	/// The names of the columns in `expenses` without any aliasing.
	pub const fn default() -> Self
	{
		Self {
			category: "category",
			cost: "cost",
			description: "description",
			id: "id",
			timesheet_id: "timesheet_id",
		}
	}
}
