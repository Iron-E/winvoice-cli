mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpenseColumns<T>
{
	pub category: T,
	pub cost: T,
	pub description: T,
	pub id: T,
	pub timesheet_id: T,
}

impl<T> ExpenseColumns<T>
{
	/// # Summary
	///
	/// Returns a [`ExpenseColumns`] which outputs all of its columns as
	/// `column_1 AS aliased_column_1`.
	pub fn r#as<TAlias>(self, aliased: ExpenseColumns<TAlias>) -> ExpenseColumns<As<TAlias, T>>
	{
		ExpenseColumns {
			category: As(self.category, aliased.category),
			cost: As(self.cost, aliased.cost),
			description: As(self.description, aliased.description),
			id: As(self.id, aliased.id),
			timesheet_id: As(self.timesheet_id, aliased.timesheet_id),
		}
	}

	/// # Summary
	///
	/// Add a [scope](Self::scope) using the [default alias](TableToSql::default_alias)
	pub fn default_scope(self) -> ExpenseColumns<WithIdentifier<T, char>>
	{
		self.scope(Self::default_alias())
	}

	/// # Summary
	///
	/// Returns a [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> ExpenseColumns<WithIdentifier<T, TAlias>>
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

	/// # Summary
	///
	/// Returns a [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> ExpenseColumns<TypeCast<TCast, T>>
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

	pub const fn unique() -> Self
	{
		Self {
			category: "unique_3_expense_category",
			cost: "unique_3_expense_cost",
			description: "unique_3_expense_description",
			id: "unique_3_expense_id",
			timesheet_id: "unique_3_expense_timesheet_id",
		}
	}
}
