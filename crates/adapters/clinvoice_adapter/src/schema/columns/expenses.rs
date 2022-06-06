use crate::fmt::{TypeCast, WithIdentifier};

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
	/// Returns an alternation of [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scoped<TIdent>(self, ident: TIdent) -> ExpenseColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		ExpenseColumns {
			id: WithIdentifier(ident, self.id),
			timesheet_id: WithIdentifier(ident, self.timesheet_id),
			category: WithIdentifier(ident, self.category),
			cost: WithIdentifier(ident, self.cost),
			description: WithIdentifier(ident, self.description),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`ExpenseColumns`] which modifies its fields' [`Display`]
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
}
