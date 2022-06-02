use core::fmt::Display;

use clinvoice_schema::{Employee, Expense, Job, Timesheet};
use sqlx::{postgres::PgRow, Row};

use crate::schema::PgScopedColumn;

pub(in crate::schema) struct PgTimesheetColumns<D>
where
	D: Display,
{
	pub employee_id: D,
	pub id: D,
	pub job_id: D,
	pub time_begin: D,
	pub time_end: D,
	pub work_notes: D,
}

impl<D> PgTimesheetColumns<D>
where
	D: Copy + Display,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgTimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgTimesheetColumns<PgScopedColumn<D, TIdent>>
	where
		TIdent: Copy + Display,
	{
		PgTimesheetColumns {
			employee_id: PgScopedColumn(ident, self.employee_id),
			id: PgScopedColumn(ident, self.id),
			job_id: PgScopedColumn(ident, self.job_id),
			time_begin: PgScopedColumn(ident, self.time_begin),
			time_end: PgScopedColumn(ident, self.time_end),
			work_notes: PgScopedColumn(ident, self.work_notes),
		}
	}
}

impl PgTimesheetColumns<&str>
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

impl PgTimesheetColumns<&'static str>
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
