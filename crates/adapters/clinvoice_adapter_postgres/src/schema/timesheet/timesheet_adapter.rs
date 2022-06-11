use std::collections::HashMap;

use clinvoice_adapter::{
	schema::{
		columns::TimesheetColumns,
		EmployeeAdapter,
		ExpensesAdapter,
		JobAdapter,
		TimesheetAdapter,
	},
	WriteWhereClause,
};
use clinvoice_finance::Money;
use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Employee,
	Job,
	Timesheet,
};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{PgPool, QueryBuilder, Result, Row};

use super::PgTimesheet;
use crate::{
	schema::{util, PgEmployee, PgExpenses, PgJob},
	PgSchema,
};

#[async_trait::async_trait]
impl TimesheetAdapter for PgTimesheet
{
	async fn create(
		connection: &PgPool,
		employee: Employee,
		expenses: Vec<(String, Money, String)>,
		job: Job,
		time_begin: DateTime<Utc>,
		time_end: Option<DateTime<Utc>>,
	) -> Result<Timesheet>
	{
		connection
			.begin()
			.and_then(|mut transaction| async {
				let work_notes =
					String::from("* Work which was done goes here\n* Supports markdown formatting");

				let row = sqlx::query!(
					"INSERT INTO timesheets
						(employee_id, job_id, time_begin, time_end, work_notes)
					VALUES
						($1,          $2,     $3,         $4,       $5)
					RETURNING id;",
					employee.id,
					job.id,
					time_begin,
					time_end,
					work_notes,
				)
				.fetch_one(&mut transaction)
				.await?;

				let expenses_db = PgExpenses::create(&mut transaction, expenses, row.id).await?;

				transaction.commit().await?;

				Ok(Timesheet {
					id: row.id,
					employee,
					expenses: expenses_db,
					job,
					time_begin: util::sanitize_datetime(time_begin),
					time_end: time_end.map(util::sanitize_datetime),
					work_notes,
				})
			})
			.await
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: &MatchTimesheet,
	) -> Result<Vec<Timesheet>>
	{
		let expenses_fut = PgExpenses::retrieve(connection, &match_condition.expenses);
		let employees_fut =
			PgEmployee::retrieve(connection, &match_condition.employee).map_ok(|vec| {
				vec.into_iter()
					.map(|e| (e.id, e))
					.collect::<HashMap<_, _>>()
			});
		let jobs_fut = PgJob::retrieve(connection, &match_condition.job).map_ok(|vec| {
			vec.into_iter()
				.map(|j| (j.id, j))
				.collect::<HashMap<_, _>>()
		});

		const COLUMNS: TimesheetColumns<&'static str> = TimesheetColumns::default();

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
		PgSchema::write_where_clause(Default::default(), "T", match_condition, &mut query);

		let (expenses, employees, jobs) = futures::try_join!(expenses_fut, employees_fut, jobs_fut)?;
		query
			.push(';')
			.build()
			.fetch(connection)
			.try_filter_map(|row| {
				if let Some(e) = employees.get(&row.get(COLUMNS.employee_id))
				{
					if let Some(x) = expenses.get(&row.get(COLUMNS.id))
					{
						if let Some(j) = jobs.get(&row.get(COLUMNS.job_id))
						{
							return future::ok(Some(PgTimesheet::row_to_view(
								COLUMNS,
								&row,
								e.clone(),
								x.clone(),
								j.clone(),
							)));
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

	use clinvoice_adapter::schema::{
		EmployeeAdapter,
		JobAdapter,
		LocationAdapter,
		OrganizationAdapter,
	};
	use clinvoice_finance::{ExchangeRates, Exchangeable};
	use clinvoice_match::{Match, MatchEmployee, MatchOrganization, MatchSet, MatchTimesheet};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		ContactKind,
		Currency,
		Expense,
		Invoice,
		InvoiceDate,
		Money,
	};

	use super::{PgTimesheet, TimesheetAdapter};
	use crate::schema::{util, PgEmployee, PgJob, PgLocation, PgOrganization};

	#[tokio::test]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
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
			None,
			Utc::now(),
			Duration::new(7640, 0),
			Invoice {
				date: None,
				hourly_rate: Money::new(13_27, 2, Currency::USD),
			},
			String::new(),
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

		let timesheet = PgTimesheet::create(
			&connection,
			employee,
			vec![(
				"Food".into(),
				Money::new(40_50, 2, Currency::USD),
				"Got fastfood".into(),
			)],
			job,
			Utc.ymd(2070, 01, 01).and_hms(01, 00, 00),
			Some(Utc.ymd(2070, 01, 01).and_hms(02, 00, 00)),
		)
		.await
		.unwrap();

		let timesheet_row = sqlx::query!(
			r#"SELECT
					employee_id,
					id,
					job_id,
					time_begin,
					time_end,
					work_notes
				FROM timesheets
				WHERE id = $1;"#,
			timesheet.id,
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		let expense_row = sqlx::query!(
			r#"SELECT
					category,
					cost,
					description,
					id,
					timesheet_id
				FROM expenses
				WHERE timesheet_id = $1;"#,
			timesheet_row.id,
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		let exchange_rates = ExchangeRates::new().await.unwrap();
		assert_eq!(timesheet.employee.id, timesheet_row.employee_id);
		assert_eq!(
			timesheet
				.expenses
				.exchange(Default::default(), &exchange_rates),
			vec![Expense {
				category: expense_row.category,
				cost: Money {
					amount: expense_row.cost.parse().unwrap(),
					..Default::default()
				},
				description: expense_row.description,
				id: expense_row.id,
				timesheet_id: timesheet.id,
			}]
		);
		assert_eq!(timesheet.job.id, timesheet_row.job_id);
		assert_eq!(timesheet.time_begin, timesheet_row.time_begin);
		assert_eq!(timesheet.time_end, timesheet_row.time_end);
		assert_eq!(timesheet.work_notes, timesheet_row.work_notes);
	}

	/// TODO: use fuzzing
	#[tokio::test]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let usa = PgLocation::create(&connection, "USA".into(), Some(earth))
			.await
			.unwrap();

		let (arizona, utah) = futures::try_join!(
			PgLocation::create(&connection, "Arizona".into(), Some(usa.clone())),
			PgLocation::create(&connection, "Utah".into(), Some(usa.clone())),
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
				None,
				Utc.ymd(1990, 07, 12).and_hms(14, 10, 00),
				Duration::from_secs(900),
				Invoice {
					date: None,
					hourly_rate: Money::new(20_00, 2, Currency::USD),
				},
				String::new(),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization2.clone(),
				Some(Utc.ymd(3000, 01, 13).and_hms(11, 30, 00)),
				Utc.ymd(3000, 01, 12).and_hms(09, 15, 42),
				Duration::from_secs(900),
				Invoice {
					date: Some(InvoiceDate {
						issued: Utc.ymd(3000, 01, 13).and_hms(11, 45, 00),
						paid: Some(Utc.ymd(3000, 01, 15).and_hms(14, 27, 00)),
					}),
					hourly_rate: Money::new(200_00, 2, Currency::JPY),
				},
				String::new(),
				"Do something".into()
			),
		)
		.unwrap();

		let (timesheet, timesheet2) = futures::try_join!(
			PgTimesheet::create(&connection, employee, Vec::new(), job, Utc::now(), None),
			PgTimesheet::create(
				&connection,
				employee2,
				vec![(
					"Flight".into(),
					Money::new(300_56, 2, Currency::USD),
					"Trip to Hawaii for research".into()
				)],
				job2,
				Utc.ymd(2022, 06, 08).and_hms(15, 27, 00),
				Some(Utc.ymd(2022, 06, 09).and_hms(07, 00, 00)),
			),
		)
		.unwrap();

		let exchange_rates = ExchangeRates::new().await.unwrap();

		assert_eq!(
			PgTimesheet::retrieve(&connection, &MatchTimesheet {
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
			.as_slice(),
			&[timesheet.exchange(Default::default(), &exchange_rates)],
		);
	}
}
