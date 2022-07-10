use clinvoice_adapter::schema::columns::{
	EmployeeColumns,
	JobColumns,
	OrganizationColumns,
	TimesheetColumns,
};
use clinvoice_finance::{Decimal, Money};
use clinvoice_schema::{Expense, Timesheet};
use sqlx::{error::UnexpectedNullError, postgres::PgRow, Error, Executor, Postgres, Result, Row};

use super::{util, PgEmployee, PgJob};

mod deletable;
mod timesheet_adapter;
mod updatable;

pub struct PgTimesheet;

impl PgTimesheet
{
	pub(super) async fn row_to_view<TEmpColumns, TJobColumns, TOrgColumns, TTimeColumns, TXpnIdent>(
		connection: impl Executor<'_, Database = Postgres>,
		columns: TimesheetColumns<TTimeColumns>,
		employee_columns: EmployeeColumns<TEmpColumns>,
		expenses_ident: TXpnIdent,
		job_columns: JobColumns<TJobColumns>,
		organization_columns: OrganizationColumns<TOrgColumns>,
		row: &PgRow,
	) -> Result<Timesheet>
	where
		TEmpColumns: AsRef<str>,
		TJobColumns: AsRef<str>,
		TOrgColumns: AsRef<str>,
		TTimeColumns: AsRef<str>,
		TXpnIdent: AsRef<str>,
	{
		let job_fut = PgJob::row_to_view(connection, job_columns, organization_columns, row);
		Ok(Timesheet {
			employee: PgEmployee::row_to_view(employee_columns, row),
			id: row.try_get(columns.id.as_ref())?,
			time_begin: row.try_get(columns.time_begin.as_ref())?,
			time_end: row.try_get(columns.time_end.as_ref())?,
			work_notes: row.try_get(columns.work_notes.as_ref())?,
			expenses: row
				.try_get(expenses_ident.as_ref())
				.and_then(|raw_expenses: Vec<(_, String, _, _, _)>| {
					raw_expenses
						.into_iter()
						.map(|(category, cost, description, id, timesheet_id)| {
							Ok(Expense {
								category,
								description,
								id,
								timesheet_id,
								cost: Money {
									amount: cost
										.parse::<Decimal>()
										.map_err(|e| util::finance_err_to_sqlx(e.into()))?,
									..Default::default()
								},
							})
						})
						.collect::<Result<Vec<_>>>()
				})
				.or_else(|e| match e
				{
					Error::ColumnDecode { source: s, .. } if s.is::<UnexpectedNullError>() =>
					{
						Ok(Vec::new())
					},
					_ => Err(e),
				})?,
			job: job_fut.await?,
		})
	}
}
