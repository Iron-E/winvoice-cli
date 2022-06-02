use core::fmt::Display;

use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::Expense;
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, Result, Row};

use crate::schema::{util, PgScopedColumn};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgExpenseColumns<D>
where
	D: Display,
{
	pub id: D,
	pub timesheet_id: D,
	pub category: D,
	pub cost: D,
	pub description: D,
}

impl<D> PgExpenseColumns<D>
where
	D: Copy + Display,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgExpenseColumns<PgScopedColumn<D, TIdent>>
	where
		TIdent: Copy + Display,
	{
		PgExpenseColumns {
			id: PgScopedColumn(ident, self.id),
			timesheet_id: PgScopedColumn(ident, self.timesheet_id),
			category: PgScopedColumn(ident, self.category),
			cost: PgScopedColumn(ident, self.cost),
			description: PgScopedColumn(ident, self.description),
		}
	}
}

impl PgExpenseColumns<&str>
{
	pub(in crate::schema) fn row_to_view(self, row: &PgRow) -> Result<Option<Expense>>
	{
		let id = match row.try_get(self.id)
		{
			Ok(id) => id,
			Err(Error::ColumnDecode {
				index: _,
				source: s,
			}) if s.is::<UnexpectedNullError>() => return Ok(None),
			Err(e) => return Err(e),
		};

		let amount = row
			.get::<String, _>(self.cost)
			.parse::<Decimal>()
			.map_err(|e| util::finance_err_to_sqlx(e.into()))?;

		Ok(Some(Expense {
			id,
			timesheet_id: row.get(self.timesheet_id),
			category: row.get(self.category),
			cost: Money {
				amount,
				..Default::default()
			},
			description: row.get(self.description),
		}))
	}
}

impl PgExpenseColumns<&'static str>
{
	pub(in crate::schema) const fn new() -> Self
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
