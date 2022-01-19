use clinvoice_adapter::{schema::TimesheetAdapter, WriteWhereClause};
use clinvoice_finance::ExchangeRates;
use clinvoice_match::MatchTimesheet;
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
		let id_match = PgLocation::retrieve_matching_ids(
			connection,
			&match_condition.employee.organization.location,
		);

		let mut query = String::from(
			"SELECT
				array_agg((C.export, C.label, C.address_id, C.email, C.phone)) AS contact_info,
				Client.name AS client_name, Client.location_id as client_location_id,
				E.organization_id as employer_id, E.person_id, E.status, E.title,
				Employer.name AS employer_name, Employer.location_id as employer_location_id,
				J.client_id, J.date_close, J.date_open, J.increment, J.invoice_date_issued, J.invoice_date_paid,
					J.invoice_hourly_rate, J.notes, J.objectives,
				P.name AS person_name,
				T.id, T.employee_id, T.job_id, T.time_begin, T.time_end, T.work_notes,
				array_agg((X1.id, X1.category, X1.cost, X1.description)) AS expenses
			FROM timesheets T
			JOIN contact_information C ON (C.employee_id = T.employee_id)
			JOIN employees E ON (E.id = T.employee_id)
			JOIN expenses X1 ON (X1.timesheet_id = T.id)
			-- WARN: we *need* `X2`. It can be bound by a where clause while allowing `X1` to be unbound
			JOIN expenses X2 ON (X2.id = X1.id)
			JOIN jobs J ON (E.id = T.employee_id)
			JOIN organizations Client ON (Client.id = J.client_id)
			JOIN organizations Employer ON (Employer.id = E.organization_id)
			JOIN people P ON (P.id = E.person_id)",
		);
		// TODO: `write_where_clause`
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
	use clinvoice_schema::{chrono::Utc, Contact, Currency, Money};

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

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve()
	{
		// TODO: write test
	}
}
