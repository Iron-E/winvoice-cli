use clinvoice_schema::{Employee, Expense, Job, Timesheet};
use sqlx::{postgres::PgRow, Row};

pub(in crate::schema) struct PgTimesheetColumns<'col>
{
	pub employee_id: &'col str,
	pub id: &'col str,
	pub job_id: &'col str,
	pub time_begin: &'col str,
	pub time_end: &'col str,
	pub work_notes: &'col str,
}

impl PgTimesheetColumns<'_>
{
	pub(in crate::schema) fn row_to_view(
		self,
		employee: Employee,
		expenses: Vec<Expense>,
		job: Job,
		row: &PgRow,
	) -> Timesheet
	{
		Timesheet {
			employee,
			expenses,
			id: row.get(self.id),
			job,
			time_begin: row.get(self.time_begin),
			time_end: row.get(self.time_end),
			work_notes: row.get(self.work_notes),
		}
	}
}

impl PgTimesheetColumns<'static>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			id: "id",
			employee_id: "employee_id",
			job_id: "job_id",
			time_begin: "time_begin",
			time_end: "time_end",
			work_notes: "work_notes",
		}
	}
}
