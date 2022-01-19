use core::str::FromStr;

use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{views::TimesheetView, Expense};
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::{employee::columns::PgEmployeeColumns, job::columns::PgJobColumns};

pub(in crate::schema) struct PgTimesheetColumns<'col>
{
	pub employee: PgEmployeeColumns<'col>,
	pub expenses: &'col str,
	pub job: PgJobColumns<'col>,
	pub time_begin: &'col str,
	pub time_end: &'col str,
	pub work_notes: &'col str,
}

impl PgTimesheetColumns<'_>
{
	pub(in crate::schema) async fn row_to_view(
		self,
		connection: &PgPool,
		row: &PgRow,
	) -> Result<TimesheetView>
	{
		let employee = self.employee.row_to_view(connection, row);
		let job = self.job.row_to_view(connection, row);

		Ok(TimesheetView {
			employee: employee.await?,
			expenses: {
				let vec: Vec<(String, String, String)> = row.get(self.expenses);
				let mut expenses = Vec::with_capacity(vec.len());
				vec.into_iter()
					.try_for_each(|(category, cost, description)| {
						Ok(expenses.push(Expense {
							category,
							cost: Money {
								amount: cost.parse()?,
								..Default::default()
							},
							description,
						}))
					})
					.and(Ok(expenses))
					.map_err(|e: <Decimal as FromStr>::Err| {
						Error::Decode(
							format!(
								"`expense.cost` is not validly formatted: {e}\nThe constraints on table \
								 `jobs` have failed"
							)
							.into(),
						)
					})?
			},
			job: job.await?,
			time_begin: row.get(self.time_begin),
			time_end: row.get(self.time_end),
			work_notes: row.get(self.work_notes),
		})
	}
}
