use core::str::FromStr;

use clinvoice_finance::{Money, Decimal};
use clinvoice_schema::{views::TimesheetView, Expense};
use sqlx::{postgres::PgRow, PgPool, Result, Row, Error};

use super::{PgEmployee, PgJob};

mod deletable;
mod timesheet_adapter;
mod updatable;

pub struct PgTimesheet;

impl PgTimesheet
{
	pub(super) async fn row_to_view(
		row: &PgRow,
		connection: &PgPool,
		client_id: &str,
		client_location_id: &str,
		client_name: &str,
		contact_info: &str,
		employee_id: &str,
		employee_name: &str,
		employee_person_id: &str,
		employee_status: &str,
		employee_title: &str,
		employer_id: &str,
		employer_location_id: &str,
		employer_name: &str,
		expenses: &str,
		invoice_date_issued: &str,
		invoice_date_paid: &str,
		invoice_hourly_rate: &str,
		job_date_close: &str,
		job_date_open: &str,
		job_id: &str,
		job_increment: &str,
		job_notes: &str,
		job_objectives: &str,
		time_begin: &str,
		time_end: &str,
		work_notes: &str,
	) -> Result<TimesheetView>
	{
		let employee = PgEmployee::row_to_view(
			row,
			connection,
			contact_info,
			employee_id,
			employee_name,
			employer_id,
			employer_location_id,
			employer_name,
			employee_person_id,
			employee_status,
			employee_title,
		);
		let job = PgJob::row_to_view(
			row,
			connection,
			job_date_close,
			job_date_open,
			job_id,
			job_increment,
			invoice_date_issued,
			invoice_date_paid,
			invoice_hourly_rate,
			job_notes,
			job_objectives,
			client_id,
			client_location_id,
			client_name,
		);

		Ok(TimesheetView {
			employee: employee.await?,
			expenses: {
				let vec: Vec<(String, String, String)> = row.get(expenses);
				let mut expenses = Vec::with_capacity(vec.len());
				vec.into_iter().try_for_each(|(category, cost, description)| {
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
							"`expense.cost` is not validly formatted: {e}\nThe constraints on \
							 table `jobs` have failed"
						)
						.into(),
					)
				})?
			},
			job: job.await?,
			time_begin: row.get(time_begin),
			time_end: row.get(time_end),
			work_notes: row.get(work_notes),
		})
	}
}
