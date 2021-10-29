use clinvoice_adapter::data::TimesheetAdapter;
use clinvoice_data::{
	chrono::{SubsecRound, Utc},
	views::TimesheetView,
	Employee,
	Job,
	Timesheet,
};
use clinvoice_query as query;
use sqlx::{postgres::Postgres, Executor, Result};

use super::PostgresTimesheet;

#[async_trait::async_trait]
impl TimesheetAdapter for PostgresTimesheet
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		employee: &Employee,
		job: &Job,
	) -> Result<Timesheet>
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
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Timesheet,
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

	use clinvoice_adapter::data::{
		EmployeeAdapter,
		Initializable,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
		PersonAdapter,
	};
	use clinvoice_data::{chrono::Utc, Contact, Currency, EmployeeStatus, Expense, Money};

	use super::{PostgresTimesheet, TimesheetAdapter};
	use crate::data::{
		util,
		PostgresEmployee,
		PostgresJob,
		PostgresLocation,
		PostgresOrganization,
		PostgresPerson,
		PostgresSchema,
	};

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&mut connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let job = PostgresJob::create(
			&mut connection,
			&organization,
			Utc::now(),
			Money::new(13_27, 2, Currency::USD),
			Duration::new(7640, 0),
			"Write the test".into(),
		)
		.await
		.unwrap();

		let person = PostgresPerson::create(&mut connection, "My Name".into())
			.await
			.unwrap();

		let mut contact_info = HashMap::new();
		contact_info.insert("Office".into(), Contact::Address {
			location_id: earth.id,
			export:      false,
		});
		contact_info.insert("Work Email".into(), Contact::Email {
			email:  "foo@bar.io".into(),
			export: true,
		});
		contact_info.insert("Office Phone".into(), Contact::Phone {
			phone:  "555 223 5039".into(),
			export: true,
		});

		let employee = PostgresEmployee::create(
			&mut connection,
			contact_info,
			&organization,
			&person,
			EmployeeStatus::Employed,
			"Janitor".into(),
		)
		.await
		.unwrap();

		let timesheet = PostgresTimesheet::create(&mut connection, &employee, &job)
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
				FROM timesheets;"#
		)
		.fetch_one(&mut connection)
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
						amount:   amnt.parse().unwrap(),
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

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
