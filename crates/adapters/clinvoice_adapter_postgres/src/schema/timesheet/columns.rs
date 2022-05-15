use core::str::FromStr;

use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{Contact, Expense, Id, Timesheet};
use sqlx::{postgres::PgRow, Error, PgPool, Result, Row};

use crate::schema::{employee::columns::PgEmployeeColumns, job::columns::PgJobColumns};

pub(in crate::schema) struct PgTimesheetColumns<'col>
{
	pub id: &'col str,
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
		contact_info: Vec<Contact>,
		row: &PgRow,
	) -> Result<Timesheet>
	{
		let employee = self.employee.row_to_view(connection, contact_info, row);
		let job = self.job.row_to_view(connection, row);

		Ok(Timesheet {
			id: row.get(self.id),
			employee: employee.await?,
			expenses: {
				match row.try_get::<Vec<(Id, String, String, String)>, _>(self.expenses)
				{
					Ok(expenses) =>
					{
						let len = expenses.len();
						expenses
							.into_iter()
							.try_fold(
								Vec::with_capacity(len),
								|mut v, (id, category, cost, description)| {
									v.push(Expense {
										id,
										category,
										cost: Money {
											amount: cost.parse()?,
											..Default::default()
										},
										description,
									});
									Ok(v)
								},
							)
							.map_err(|e: <Decimal as FromStr>::Err| {
								Error::Decode(
									format!(
										"`expense.cost` is not validly formatted: {e}\nThe constraints on \
										 table `jobs` have failed"
									)
									.into(),
								)
							})?
					},
					Err(Error::ColumnNotFound(_)) => Vec::with_capacity(0),
					Err(e) => return Err(e),
				}
			},
			job: job.await?,
			time_begin: row.get(self.time_begin),
			time_end: row.get(self.time_end),
			work_notes: row.get(self.work_notes),
		})
	}
}
