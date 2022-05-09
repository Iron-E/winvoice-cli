use clinvoice_adapter::{schema::TimesheetAdapter, WriteWhereClause};
use clinvoice_finance::ExchangeRates;
use clinvoice_match::{MatchExpense, MatchTimesheet};
use clinvoice_schema::{
	chrono::{SubsecRound, Utc},
	Employee,
	Job,
	Timesheet,
};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{PgPool, Result};

use super::{columns::PgTimesheetColumns, PgTimesheet};
use crate::{
	schema::{
		employee::columns::PgEmployeeColumns,
		job::columns::PgJobColumns,
		organization::columns::PgOrganizationColumns,
		person::columns::PgPersonColumns,
		util,
		PgLocation,
	},
	PgSchema as Schema,
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
		let exchange_rates = ExchangeRates::new().map_err(util::finance_err_to_sqlx);
		let client_location_id_match =
			PgLocation::retrieve_matching_ids(connection, &match_condition.job.client.location);
		let employer_location_id_match = PgLocation::retrieve_matching_ids(
			connection,
			&match_condition.employee.organization.location,
		);

		let mut query = String::from(
			"SELECT
				-- FIX: use latest PostgresEmployee code
				array_agg((C1.export, C1.label, C1.address_id, C1.email, C1.phone)) AS contact_info,
				Client.name AS client_name, Client.location_id as client_location_id,
				E.organization_id as employer_id, E.person_id, E.status, E.title,
				Employer.name AS employer_name, Employer.location_id as employer_location_id,
				J.client_id, J.date_close, J.date_open, J.increment, J.invoice_date_issued, J.invoice_date_paid,
					J.invoice_hourly_rate, J.notes, J.objectives,
				P.name AS person_name,
				T.id, T.employee_id, T.job_id, T.time_begin, T.time_end, T.work_notes,
				array_agg(DISTINCT (X1.id, X1.category, X1.cost, X1.description)) AS expenses
			FROM timesheets T
			JOIN contact_information C1 ON (C1.employee_id = T.id)
			JOIN employees E ON (E.id = T.employee_id)
			JOIN expenses X1 ON (X1.timesheet_id = T.id)
			JOIN jobs J ON (E.id = T.employee_id)
			JOIN organizations Client ON (Client.id = J.client_id)
			JOIN organizations Employer ON (Employer.id = E.organization_id)
			JOIN people P ON (P.id = E.person_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					Schema::write_where_clause(
						Schema::write_where_clause(
							Schema::write_where_clause(
								Schema::write_where_clause(
									Schema::write_where_clause(
										Schema::write_where_clause(
											crate::schema::write_where_clause::write_contact_set_where_clause(
												connection,
												Default::default(),
												"C1",
												&match_condition.employee.contact_info,
												&mut query,
											)
											.await?,
											"Client",
											&match_condition.job.client,
											&mut query,
										),
										"E",
										&match_condition.employee,
										&mut query,
									),
									"Employer",
									&match_condition.employee.organization,
									&mut query,
								),
								"J",
								&match_condition.job,
								&mut query,
							),
							"P",
							&match_condition.employee.person,
							&mut query,
						),
						"T",
						&match_condition,
						&mut query,
					),
					"X1",
					{
						let rates = exchange_rates.await?;
						&match_condition.expenses.map(&|e| MatchExpense {
							id: e.id,
							category: e.category,
							cost: e.cost.exchange(Default::default(), &rates),
							description: e.description,
						})
					},
					&mut query,
				),
				"Client.location_id",
				&client_location_id_match.await?,
				&mut query,
			),
			"Employer.location_id",
			&employer_location_id_match.await?,
			&mut query,
		);
		query.push_str(
			" GROUP BY
				Client.name, Client.location_id,
				E.organization_id, E.person_id, E.status, E.title,
				Employer.name, Employer.location_id,
				J.client_id, J.date_close, J.date_open, J.increment, J.invoice_date_issued, J.invoice_date_paid,
					J.invoice_hourly_rate, J.notes, J.objectives,
				P.name,
				T.id, T.employee_id, T.job_id, T.time_begin, T.time_end, T.work_notes
			;",
		);

		const COLUMNS: PgTimesheetColumns<'static> = PgTimesheetColumns {
			id: "id",
			employee: PgEmployeeColumns {
				contact_info: "contact_information",
				id: "employee_id",
				organization: PgOrganizationColumns {
					id: "employer_id",
					location_id: "employer_location_id",
					name: "employer_name",
				},
				person: PgPersonColumns {
					id: "person_id",
					name: "person_name",
				},
				status: "status",
				title: "title",
			},
			expenses: "expenses",
			job: PgJobColumns {
				client: PgOrganizationColumns {
					id: "client_id",
					location_id: "client_location_id",
					name: "client_name",
				},
				id: "job_id",
			},
			time_begin: "time_begin",
			time_end: "time_end",
			work_notes: "work_notes",
		};

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move { COLUMNS.row_to_view(connection, &row).await })
			.try_collect()
			.await
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
	use clinvoice_match::{MatchEmployee, MatchPerson, MatchTimesheet, MatchExpense, MatchSet};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		Contact,
		Currency,
		Money,
	};

	use super::{PgTimesheet, TimesheetAdapter};
	use crate::schema::{util, PgEmployee, PgJob, PgLocation, PgOrganization, PgPerson};

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into())
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

		let person = PgPerson::create(&connection, "My Name".into())
			.await
			.unwrap();

		let mut contact_info = HashMap::new();
		contact_info.insert("Office".into(), Contact::Address {
			location: earth,
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
			organization,
			person,
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

		let (person, person2) = futures::try_join!(
			PgPerson::create(&connection, "My Name".into()),
			PgPerson::create(&connection, "Another Gúy".into()),
		)
		.unwrap();

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
			PgOrganization::create(&connection, arizona.clone(), "Some Organization".into()),
			PgOrganization::create(&connection, utah.clone(), "Some Other Organizatión".into()),
		)
		.unwrap();

		let (employee, employee2) = futures::try_join!(
			PgEmployee::create(
				&connection,
				[
					("Remote Office".into(), Contact::Address {
						location: utah,
						export: false,
					}),
					("Work Email".into(), Contact::Email {
						email: "foo@bar.io".into(),
						export: true,
					}),
					("Office's Phone".into(), Contact::Phone {
						phone: "555 223 5039".into(),
						export: true,
					}),
				]
				.into_iter()
				.collect(),
				organization.clone(),
				person.clone(),
				"Employed".into(),
				"Janitor".into(),
			),
			PgEmployee::create(
				&connection,
				[
					("Favorite Pizza Place".into(), Contact::Address {
						location: arizona,
						export: false,
					}),
					("Work Email".into(), Contact::Email {
						email: "some_kind_of_email@f.com".into(),
						export: true,
					}),
					("Office's Phone".into(), Contact::Phone {
						phone: "555-555-8008".into(),
						export: true,
					}),
				]
				.into_iter()
				.collect(),
				organization2.clone(),
				person2,
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
				expenses: MatchSet::Contains(MatchExpense {
				}),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[timesheet],
		);
	}
}
