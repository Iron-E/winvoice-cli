use clinvoice_adapter::schema::TimesheetAdapter;
use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{
	chrono::{SubsecRound, Utc},
	views::TimesheetView,
	Employee,
	Job,
	Timesheet,
};
use sqlx::{PgPool, Result};

use super::PgTimesheet;

#[async_trait::async_trait]
impl TimesheetAdapter for PgTimesheet
{
	async fn create(connection: &PgPool, employee: &Employee, job: &Job) -> Result<Timesheet>
	{
		let time_begin = Utc::now();
		let work_notes =
			String::from("* Work which was done goes here\n* Supports markdown formatting");

		sqlx::query!(
			"INSERT INTO timesheets
				(employee_id, job_id, expenses,           time_begin, work_notes)
			VALUES
				($1,          $2,     ARRAY[]::expense[], $3,         $4)
			;",
			employee.id,
			job.id,
			time_begin,
			work_notes,
		)
		.execute(connection)
		.await?;

		Ok(Timesheet {
			employee_id: employee.id,
			expenses: Vec::new(),
			job_id: job.id,
			time_begin: time_begin.trunc_subsecs(6),
			time_end: None,
			work_notes,
		})
	}

	async fn retrieve_view(
		connection: &PgPool,
		match_condition: &MatchTimesheet,
	) -> Result<Vec<TimesheetView>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;
	use std::collections::HashMap;

	use clinvoice_adapter::schema::{
		EmployeeAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
		PersonAdapter,
	};
	use clinvoice_schema::{chrono::Utc, Contact, Currency, Expense, Money};

	use super::{PgTimesheet, TimesheetAdapter};
	use crate::schema::{util, PgEmployee, PgJob, PgLocation, PgOrganization, PgPerson};

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization = PgOrganization::create(&connection, &earth, "Some Organization".into())
			.await
			.unwrap();

		let job = PgJob::create(
			&connection,
			&organization,
			Utc::now(),
			Money::new(13_27, 2, Currency::USD),
			Duration::new(7640, 0),
			"Write the test".into(),
		)
		.await
		.unwrap();

		let person = PgPerson::create(&connection, "My Name".into())
			.await
			.unwrap();

		let mut contact_info = HashMap::new();
		contact_info.insert("Office".into(), Contact::Address {
			location_id: earth.id,
			export: false,
		});
		contact_info.insert("Work Email".into(), Contact::Email {
			email: "foo@bar.io".into(),
			export: true,
		});
		contact_info.insert("Office Phone".into(), Contact::Phone {
			phone: "555 223 5039".into(),
			export: true,
		});

		let employee = PgEmployee::create(
			&connection,
			contact_info,
			&organization,
			&person,
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let timesheet = PgTimesheet::create(&connection, &employee, &job)
			.await
			.unwrap();

		let row = sqlx::query!(
			r#"SELECT
					employee_id,
					job_id,
					expenses as "expenses: Vec<(String, (String, String), String)>",
					time_begin,
					time_end,
					work_notes
				FROM timesheets
				WHERE time_begin = $1;"#,
			timesheet.time_begin,
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		assert_eq!(timesheet.employee_id, row.employee_id);
		assert_eq!(
			timesheet.expenses,
			row.expenses
				.into_iter()
				.map(|(ctg, (amnt, curr), description)| Expense {
					category: ctg.parse().unwrap(),
					cost: Money {
						amount: amnt.parse().unwrap(),
						currency: curr.parse().unwrap(),
					},
					description
				})
				.collect::<Vec<_>>()
		);
		assert_eq!(timesheet.job_id, row.job_id);
		assert_eq!(timesheet.time_begin, row.time_begin);
		assert_eq!(timesheet.time_end, row.time_end);
		assert_eq!(timesheet.work_notes, row.work_notes);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
