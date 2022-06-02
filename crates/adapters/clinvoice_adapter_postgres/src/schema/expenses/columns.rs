use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::Expense;
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, Result, Row};

use crate::schema::util;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgExpenseColumns<'col>
{
	pub id: &'col str,
	pub timesheet_id: &'col str,
	pub category: &'col str,
	pub cost: &'col str,
	pub description: &'col str,
}

impl PgExpenseColumns<'_>
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

impl PgExpenseColumns<'static>
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
