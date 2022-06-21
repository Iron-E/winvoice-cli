mod columns_to_sql;

use crate::fmt::{TypeCast, WithIdentifier};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimesheetColumns<T>
{
	pub employee_id: T,
	pub id: T,
	pub job_id: T,
	pub time_begin: T,
	pub time_end: T,
	pub work_notes: T,
}

impl<T> TimesheetColumns<T>
{
	/// # Summary
	///
	/// Returns an alternation of [`TimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scoped<TIdent>(self, ident: TIdent) -> TimesheetColumns<WithIdentifier<T, TIdent>>
	where
		TIdent: Copy,
	{
		TimesheetColumns {
			employee_id: WithIdentifier(ident, self.employee_id),
			id: WithIdentifier(ident, self.id),
			job_id: WithIdentifier(ident, self.job_id),
			time_begin: WithIdentifier(ident, self.time_begin),
			time_end: WithIdentifier(ident, self.time_end),
			work_notes: WithIdentifier(ident, self.work_notes),
		}
	}

	/// # Summary
	///
	/// Returns an alternation of [`TimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> TimesheetColumns<TypeCast<TCast, T>>
	where
		TCast: Copy,
	{
		TimesheetColumns {
			employee_id: TypeCast(self.employee_id, cast),
			id: TypeCast(self.id, cast),
			job_id: TypeCast(self.job_id, cast),
			time_begin: TypeCast(self.time_begin, cast),
			time_end: TypeCast(self.time_end, cast),
			work_notes: TypeCast(self.work_notes, cast),
		}
	}
}

impl TimesheetColumns<&'static str>
{
	pub const fn default() -> Self
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

	pub const fn unique() -> Self
	{
		Self {
			id: "unique_7_timesheet_id",
			employee_id: "unique_7_timesheet_employee_id",
			job_id: "unique_7_timesheet_job_id",
			time_begin: "unique_7_timesheet_time_begin",
			time_end: "unique_7_timesheet_time_end",
			work_notes: "unique_7_timesheet_work_notes",
		}
	}
}
