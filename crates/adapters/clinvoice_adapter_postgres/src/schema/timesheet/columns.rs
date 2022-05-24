use clinvoice_schema::{Contact, Expense, Timesheet};
use sqlx::{postgres::PgRow, PgPool, Result, Row};

use crate::schema::{employee::columns::PgEmployeeColumns, job::columns::PgJobColumns};

pub(in crate::schema) struct PgTimesheetColumns<'col>
{
	pub id: &'col str,
	pub employee: PgEmployeeColumns<'col>,
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
		expenses: Vec<Expense>,
		row: &PgRow,
	) -> Result<Timesheet>
	{
		let employee = self.employee.row_to_view(connection, contact_info, row);
		let job = self.job.row_to_view(connection, row);

		Ok(Timesheet {
			expenses,
			id: row.get(self.id),
			time_begin: row.get(self.time_begin),
			time_end: row.get(self.time_end),
			work_notes: row.get(self.work_notes),
			employee: employee.await?,
			job: job.await?,
		})
	}
}
