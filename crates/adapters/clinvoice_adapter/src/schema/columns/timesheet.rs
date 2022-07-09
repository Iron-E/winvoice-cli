mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

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
	/// Returns a [`TimesheetColumns`] which outputs all of its columns as
	/// `column_1 AS aliased_column_1`.
	pub fn r#as<TAlias>(self, aliased: TimesheetColumns<TAlias>) -> TimesheetColumns<As<T, TAlias>>
	{
		TimesheetColumns {
			employee_id: As(self.employee_id, aliased.employee_id),
			id: As(self.id, aliased.id),
			job_id: As(self.job_id, aliased.job_id),
			time_begin: As(self.time_begin, aliased.time_begin),
			time_end: As(self.time_end, aliased.time_end),
			work_notes: As(self.work_notes, aliased.work_notes),
		}
	}

	/// # Summary
	///
	/// Add a [scope](Self::scope) using the [default alias](TableToSql::default_alias)
	pub fn default_scope(self) -> TimesheetColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// # Summary
	///
	/// Returns a [`TimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{ident}.{column}`.
	pub fn scope<TAlias>(self, alias: TAlias) -> TimesheetColumns<WithIdentifier<TAlias, T>>
	where
		TAlias: Copy,
	{
		TimesheetColumns {
			employee_id: WithIdentifier(alias, self.employee_id),
			id: WithIdentifier(alias, self.id),
			job_id: WithIdentifier(alias, self.job_id),
			time_begin: WithIdentifier(alias, self.time_begin),
			time_end: WithIdentifier(alias, self.time_end),
			work_notes: WithIdentifier(alias, self.work_notes),
		}
	}

	/// # Summary
	///
	/// Returns a [`TimesheetColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	pub fn typecast<TCast>(self, cast: TCast) -> TimesheetColumns<TypeCast<T, TCast>>
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
