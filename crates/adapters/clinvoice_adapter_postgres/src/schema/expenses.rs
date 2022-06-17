use clinvoice_adapter::schema::columns::ExpenseColumns;
use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::Expense;
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, Result, Row};

use super::util;

mod deletable;
mod expenses_adapter;
mod updatable;

pub struct PgExpenses;

impl PgExpenses
{
	pub(super) fn row_to_view(columns: ExpenseColumns<&str>, row: &PgRow)
		-> Result<Option<Expense>>
	{
		let id = match row.try_get(columns.id)
		{
			Ok(id) => id,
			Err(Error::ColumnDecode {
				index: _,
				source: s,
			}) if s.is::<UnexpectedNullError>() => return Ok(None),
			Err(e) => return Err(e),
		};

		let amount = row
			.get::<String, _>(columns.cost)
			.parse::<Decimal>()
			.map_err(|e| util::finance_err_to_sqlx(e.into()))?;

		Ok(Some(Expense {
			id,
			timesheet_id: row.get(columns.timesheet_id),
			category: row.get(columns.category),
			cost: Money {
				amount,
				..Default::default()
			},
			description: row.get(columns.description),
		}))
	}
}
