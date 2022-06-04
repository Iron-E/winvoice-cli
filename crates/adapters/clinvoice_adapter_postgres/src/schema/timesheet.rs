use clinvoice_adapter::schema::columns::TimesheetColumns;
use clinvoice_schema::{Employee, Expense, Job, Timesheet};
use sqlx::{postgres::PgRow, Row};

mod deletable;
mod timesheet_adapter;
mod updatable;

pub struct PgTimesheet;

impl PgTimesheet
{
	pub(in crate::schema) fn row_to_view(
		columns: TimesheetColumns<&str>,
		row: &PgRow,
		employee: Employee,
		expenses: Vec<Expense>,
		job: Job,
	) -> Timesheet
	{
		Timesheet {
			employee,
			expenses,
			id: row.get(columns.id),
			job,
			time_begin: row.get(columns.time_begin),
			time_end: row.get(columns.time_end),
			work_notes: row.get(columns.work_notes),
		}
	}
}
