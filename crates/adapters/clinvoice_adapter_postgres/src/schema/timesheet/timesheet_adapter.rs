use clinvoice_adapter::{
	fmt::{ColumnsToSql, QueryBuilderExt},
	schema::{
		columns::{
			EmployeeColumns,
			ExpenseColumns,
			JobColumns,
			LocationColumns,
			OrganizationColumns,
			TimesheetColumns,
		},
		ExpensesAdapter,
		TimesheetAdapter,
	},
	WriteWhereClause,
};
use clinvoice_finance::{ExchangeRates, Exchangeable, Money};
use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Employee,
	Job,
	Timesheet,
};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{PgPool, Result};

use super::PgTimesheet;
use crate::{
	fmt::PgLocationRecursiveCte,
	schema::{util, PgExpenses, PgLocation},
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
					"* Work which was done goes here\n* Supports markdown formatting".to_string();

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
		const ALIAS: &str = "T";
		const COLUMNS: TimesheetColumns<&str> = TimesheetColumns::default();

		const EMPLOYEE_ALIAS: &str = "E";
		const EMPLOYEE_COLUMNS: EmployeeColumns<&str> = EmployeeColumns::default();
		const EMPLOYEE_COLUMNS_UNIQUE: EmployeeColumns<&str> = EmployeeColumns::unique();

		const EXPENSE_ALIAS: &str = "X";
		const EXPENSES_AGGREGATED_IDENT: &str = "expenses_aggregated";
		const EXPENSE_COLUMNS: ExpenseColumns<&str> = ExpenseColumns::default();

		const JOB_ALIAS: &str = "J";
		const JOB_COLUMNS: JobColumns<&str> = JobColumns::default();
		const JOB_COLUMNS_UNIQUE: JobColumns<&str> = JobColumns::unique();

		const LOCATION_ALIAS: &str = "L";
		const LOCATION_COLUMNS: LocationColumns<&str> = LocationColumns::default();

		const ORGANIZATION_ALIAS: &str = "O";
		const ORGANIZATION_COLUMNS: OrganizationColumns<&str> = OrganizationColumns::default();
		const ORGANIZATION_COLUMNS_UNIQUE: OrganizationColumns<&str> = OrganizationColumns::unique();

		let columns = COLUMNS.scope(ALIAS);
		let employee_columns = EMPLOYEE_COLUMNS.scope(EMPLOYEE_ALIAS);
		let exchange_rates_fut = ExchangeRates::new().map_err(util::finance_err_to_sqlx);
		let expense_columns = EXPENSE_COLUMNS.scope(EXPENSE_ALIAS);
		let job_columns = JOB_COLUMNS.scope(JOB_ALIAS);
		let location_columns = LOCATION_COLUMNS.scope(LOCATION_ALIAS);
		let mut query = PgLocation::query_with_recursive(&match_condition.job.client.location);
		let organization_columns = ORGANIZATION_COLUMNS.scope(ORGANIZATION_ALIAS);

		query.push("SELECT ");
		columns.push_to(&mut query);

		query.push(',');
		employee_columns
			.r#as(EMPLOYEE_COLUMNS_UNIQUE)
			.push_to(&mut query);

		// NOTE: might need `",array_agg( DISTINCT ("`
		query.push(",array_agg((");
		expense_columns.push_to(&mut query);
		query
			.separated(' ')
			.push(")) AS")
			.push(EXPENSES_AGGREGATED_IDENT);

		query.push(',');
		job_columns.r#as(JOB_COLUMNS_UNIQUE).push_to(&mut query);

		query.push(',');
		organization_columns
			.r#as(ORGANIZATION_COLUMNS_UNIQUE)
			.push_to(&mut query);

		query
			.push_from("timesheets", ALIAS)
			.push_equijoin(
				"employees",
				EMPLOYEE_ALIAS,
				employee_columns.id,
				columns.employee_id,
			)
			.push(" LEFT")
			.push_equijoin(
				"expenses",
				EXPENSE_ALIAS,
				expense_columns.timesheet_id,
				columns.id,
			)
			.push_equijoin("jobs", JOB_ALIAS, job_columns.id, columns.job_id)
			.push_equijoin(
				"organizations",
				ORGANIZATION_ALIAS,
				organization_columns.id,
				job_columns.client_id,
			)
			.push_equijoin(
				PgLocationRecursiveCte::from(&match_condition.job.client.location),
				LOCATION_ALIAS,
				location_columns.id,
				organization_columns.location_id,
			);

		let exchange_rates = exchange_rates_fut.await?;
		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					PgSchema::write_where_clause(
						PgSchema::write_where_clause(
							Default::default(),
							ALIAS,
							match_condition,
							&mut query,
						),
						EMPLOYEE_ALIAS,
						&match_condition.employee,
						&mut query,
					),
					EXPENSE_ALIAS,
					&match_condition
						.expenses
						.exchange_ref(Default::default(), &exchange_rates),
					&mut query,
				),
				JOB_ALIAS,
				&match_condition
					.job
					.exchange_ref(Default::default(), &exchange_rates),
				&mut query,
			),
			ORGANIZATION_ALIAS,
			&match_condition.job.client,
			&mut query,
		);

		query
			.push(" GROUP BY ")
			.separated(',')
			.push(columns.id)
			.push(employee_columns.id)
			.push(job_columns.id)
			.push(organization_columns.id);

		query
			.push(';')
			.build()
			.fetch(connection)
			.and_then(|row| async move {
				PgTimesheet::row_to_view(
					connection,
					COLUMNS,
					EMPLOYEE_COLUMNS_UNIQUE,
					EXPENSES_AGGREGATED_IDENT,
					JOB_COLUMNS_UNIQUE,
					ORGANIZATION_COLUMNS_UNIQUE,
					&row,
				)
				.await
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
	use clinvoice_match::{Match, MatchEmployee, MatchSet, MatchTimesheet};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
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

		let organization = PgOrganization::create(&connection, earth, "Some Organization".into())
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
			PgOrganization::create(&connection, arizona.clone(), "Some Organization".into()),
			PgOrganization::create(&connection, utah, "Some Other Organizatión".into()),
		)
		.unwrap();

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				"My Name".into(),
				"Employed".into(),
				"Janitor".into()
			),
			PgEmployee::create(
				&connection,
				"Another Gúy".into(),
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
					id: Match::Or(vec![
						timesheet.employee.id.into(),
						timesheet2.employee.id.into(),
					]),
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
