use core::fmt::Display;

use clinvoice_schema::{Employee, Expense, Job, Timesheet};
use sqlx::{postgres::PgRow, Row};

use crate::schema::{typecast::PgTypeCast, PgScopedColumn};

pub(in crate::schema) struct PgTimesheetColumns<T>
{
	pub employee_id: T,
	pub id: T,
	pub job_id: T,
	pub time_begin: T,
	pub time_end: T,
	pub work_notes: T,
}

impl<T> PgTimesheetColumns<T>
where
	T: Copy,
{
	/// # Summary
	///
	/// Returns an alternation of [`PgTimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub(in crate::schema) fn scoped<TIdent>(
		&self,
		ident: TIdent,
	) -> PgTimesheetColumns<PgScopedColumn<T, TIdent>>
	where
		TIdent: Copy,
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

	/// # Summary
	///
	/// Returns an alternation of [`PgTimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub(in crate::schema) fn typecast<TCast>(
		&self,
		cast: TCast,
	) -> PgTimesheetColumns<PgTypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		PgTimesheetColumns {
			employee_id: PgTypeCast(self.employee_id, cast),
			id: PgTypeCast(self.id, cast),
			job_id: PgTypeCast(self.job_id, cast),
			time_begin: PgTypeCast(self.time_begin, cast),
			time_end: PgTypeCast(self.time_end, cast),
			work_notes: PgTypeCast(self.work_notes, cast),
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
