use std::collections::HashMap;

use clinvoice_adapter::{
	schema::{EmployeeAdapter, ExpensesAdapter, JobAdapter, TimesheetAdapter},
	WriteWhereClause,
};
use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{
	chrono::{SubsecRound, Utc},
	Employee,
	Job,
	Timesheet,
};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{PgPool, QueryBuilder, Result, Row};

use super::{columns::PgTimesheetColumns, PgTimesheet};
use crate::{
	schema::{PgEmployee, PgExpenses, PgJob},
	PgSchema,
};

#[async_trait::async_trait]
impl TimesheetAdapter for PgTimesheet
{
	async fn create(connection: &PgPool, employee: Employee, job: Job) -> Result<Timesheet>
	{
		let time_begin = Utc::now();
		let work_notes =
			String::from("* Work which was done goes here\n* Supports markdown formatting");

		let row = sqlx::query!(
			"INSERT INTO timesheets
				(employee_id, job_id, time_begin, work_notes)
			VALUES
				($1,          $2,     $3,         $4)
			RETURNING id;",
			employee.id,
			job.id,
			time_begin,
			work_notes,
		)
		.fetch_one(connection)
		.await?;

		Ok(Timesheet {
			id: row.id,
			employee,
			expenses: Vec::new(),
			job,
			time_begin: time_begin.trunc_subsecs(6),
			time_end: None,
			work_notes,
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: MatchTimesheet)
		-> Result<Vec<Timesheet>>
	{
		let expenses_fut = PgExpenses::retrieve(connection, match_condition.expenses.clone());
		let employees_fut = PgEmployee::retrieve(connection, match_condition.employee.clone())
			.map_ok(|vec| {
				vec.into_iter()
					.map(|e| (e.id, e))
					.collect::<HashMap<_, _>>()
			});
		let jobs_fut = PgJob::retrieve(connection, match_condition.job.clone()).map_ok(|vec| {
			vec.into_iter()
				.map(|j| (j.id, j))
				.collect::<HashMap<_, _>>()
		});

		let mut query = QueryBuilder::new(
			"SELECT
				T.id,
				T.employee_id,
				T.job_id,
				T.time_begin,
				T.time_end,
				T.work_notes
			FROM timesheets T",
		);
		PgSchema::write_where_clause(Default::default(), "T", &match_condition, &mut query);
		query.push(';');

		const COLUMNS: PgTimesheetColumns<'static> = PgTimesheetColumns {
			id: "id",
			employee_id: "employee_id",
			job_id: "job_id",
			time_begin: "time_begin",
			time_end: "time_end",
			work_notes: "work_notes",
		};

		let expenses = &expenses_fut.await?;
		let employees = &employees_fut.await?;
		let jobs = &jobs_fut.await?;
		query
			.build()
			.fetch(connection)
			.try_filter_map(|row| {
				if let Some(e) = employees.get(&row.get(COLUMNS.employee_id))
				{
					if let Some(x) = expenses.get(&row.get(COLUMNS.id))
					{
						if let Some(j) = jobs.get(&row.get(COLUMNS.job_id))
						{
							return match COLUMNS.row_to_view(e.clone(), x.clone(), j.clone(), &row)
							{
								Ok(t) => future::ok(Some(t)),
								Err(e) => future::err(e),
							};
						}
					}
				}

				future::ok(None)
			})
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;
	use std::collections::HashSet;

	use clinvoice_adapter::schema::{
		EmployeeAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
	};
	use clinvoice_match::{Match, MatchEmployee, MatchOrganization, MatchSet, MatchTimesheet};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		ContactKind,
		Currency,
		Money,
	};

	use super::{PgTimesheet, TimesheetAdapter};
	use crate::schema::{util, PgEmployee, PgJob, PgLocation, PgOrganization};

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization = PgOrganization::create(
			&connection,
			vec![
				(false, ContactKind::Address(earth.clone()), "Office".into()),
				(
					true,
					ContactKind::Email("foo@bar.io".into()),
					"Work Email".into(),
				),
				(
					true,
					ContactKind::Phone("555 223 5039".into()),
					"Office Phone".into(),
				),
			],
			earth,
			"Some Organization".into(),
		)
		.await
		.unwrap();

		let job = PgJob::create(
			&connection,
			organization.clone(),
			Utc::now(),
			Money::new(13_27, 2, Currency::USD),
			Duration::new(7640, 0),
			"Write the test".into(),
		)
		.await
		.unwrap();

		let employee = PgEmployee::create(
			&connection,
			"My Name".into(),
			organization,
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let timesheet = PgTimesheet::create(&connection, employee, job)
			.await
			.unwrap();

		let row = sqlx::query!(
			r#"SELECT
					employee_id,
					job_id,
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

		assert_eq!(timesheet.employee.id, row.employee_id);
		assert_eq!(timesheet.job.id, row.job_id);
		assert_eq!(timesheet.time_begin, row.time_begin);
		assert_eq!(timesheet.time_end, row.time_end);
		assert_eq!(timesheet.work_notes, row.work_notes);
	}

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();
		let usa = PgLocation::create_inner(&connection, earth, "USA".into())
			.await
			.unwrap();
		let (arizona, utah) = futures::try_join!(
			PgLocation::create_inner(&connection, usa.clone(), "Arizona".into()),
			PgLocation::create_inner(&connection, usa.clone(), "Utah".into()),
		)
		.unwrap();

		let (organization, organization2) = futures::try_join!(
			PgOrganization::create(
				&connection,
				vec![
					(
						false,
						ContactKind::Address(utah.clone()),
						"Remote Office".into()
					),
					(
						true,
						ContactKind::Email("foo@bar.io".into()),
						"Work Email".into(),
					),
					(
						true,
						ContactKind::Phone("555 223 5039".into()),
						"Office's Phone".into(),
					),
				],
				arizona.clone(),
				"Some Organization".into()
			),
			PgOrganization::create(
				&connection,
				vec![
					(
						false,
						ContactKind::Address(arizona),
						"Favorite Pizza Place".into()
					),
					(
						true,
						ContactKind::Email("some_kind_of_email@f.com".into()),
						"Work Email".into(),
					),
					(
						true,
						ContactKind::Phone("555-555-8008".into()),
						"Office's Phone".into(),
					),
				],
				utah,
				"Some Other Organizatión".into()
			),
		)
		.unwrap();

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				"My Name".into(),
				organization.clone(),
				"Employed".into(),
				"Janitor".into(),
			),
			PgEmployee::create(
				&connection,
				"Another Gúy".into(),
				organization2.clone(),
				"Management".into(),
				"Assistant to Regional Manager".into(),
			),
		)
		.unwrap();

		let (job, job2) = futures::try_join!(
			PgJob::create(
				&connection,
				organization.clone(),
				Utc.ymd(1990, 07, 12).and_hms(14, 10, 00),
				Money::new(20_00, 2, Currency::USD),
				Duration::from_secs(900),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization2.clone(),
				Utc.ymd(3000, 01, 12).and_hms(09, 15, 42),
				Money::new(200_00, 2, Currency::JPY),
				Duration::from_secs(900),
				"Do something".into()
			),
		)
		.unwrap();

		let (timesheet, timesheet2) = futures::try_join!(
			PgTimesheet::create(&connection, employee, job),
			PgTimesheet::create(&connection, employee2, job2),
		)
		.unwrap();

		assert_eq!(
			PgTimesheet::retrieve(&connection, MatchTimesheet {
				expenses: MatchSet::Not(MatchSet::Contains(Default::default()).into()),
				employee: MatchEmployee {
					organization: MatchOrganization {
						id: Match::Or(vec![
							timesheet.employee.organization.id.into(),
							timesheet2.employee.organization.id.into(),
						]),
						..Default::default()
					},
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>(),
			[timesheet, timesheet2].into_iter().collect(),
		);
	}
}
