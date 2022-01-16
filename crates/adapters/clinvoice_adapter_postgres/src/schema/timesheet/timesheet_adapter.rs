use std::{collections::HashMap, str::FromStr};

use clinvoice_adapter::{schema::TimesheetAdapter, WriteWhereClause};
use clinvoice_finance::{Decimal, ExchangeRates, Money};
use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{
	chrono::{SubsecRound, Utc},
	views::{ContactView, EmployeeView, JobView, OrganizationView, PersonView, TimesheetView},
	Employee,
	Expense,
	Invoice,
	InvoiceDate,
	Job,
	Timesheet,
};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{Error, PgPool, Result, Row};

use super::PgTimesheet;
use crate::{
	schema::{util, PgLocation},
	PgSchema as Schema,
};

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
		let exchange_rates = ExchangeRates::new().map_err(util::finance_err_to_sqlx);
		let id_match = PgLocation::retrieve_matching_ids(
			connection,
			&match_condition.employee.organization.location,
		);

		let mut query = String::from(
			"SELECT
				array_agg((C.export, C.label, C.address_id, C.email, C.phone)) AS contact_info,
				Client.name AS client_name, CLient.location_id as client_location_id,
				E.organization_id as employer_id, E.person_id, E.status, E.title,
				Employer.name AS employer_name, Employer.location_id as employer_location_id,
				J.client_id, J.date_close, J.date_open, J.increment, J.invoice_date_issued, J.invoice_date_paid,
					J.invoice_hourly_rate, J.notes, J.objectives,
				P.name AS person_name,
				T.employee_id, T.job_id, T.expenses, T.time_begin, T.time_end, T.work_notes
			FROM timesheets T
			JOIN contact_information C ON (C.employee_id = T.employee_id)
			JOIN employees E ON (E.id = T.employee_id)
			JOIN jobs J ON (E.id = T.employee_id)
			JOIN organizations Client ON (O.id = J.client_id)
			JOIN organizations Employer ON (Employer.id = E.organization_id)
			JOIN people P ON (P.id = E.person_id)
			",
		);
		// TODO: `write_where_clause`
		query.push(';');

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move {
				Ok(TimesheetView {
					employee: EmployeeView {
						id: row.get("employee_id"),
						organization: OrganizationView {
							id: row.get("employer_id"),
							name: row.get("employer_name"),
							location: PgLocation::retrieve_view_by_id(
								connection,
								row.get("employer_location_id"),
							)
							.await?,
						},
						person: PersonView {
							id: row.get("person_id"),
							name: row.get("person_name"),
						},
						contact_info: {
							let vec: Vec<(_, _, _, _, _)> = row.get("contact_info");
							let mut map = HashMap::with_capacity(vec.len());
							for contact in vec
							{
								map.insert(
									contact.1,
									if let Some(id) = contact.2
									{
										ContactView::Address {
											location: PgLocation::retrieve_view_by_id(connection, id).await?,
											export: contact.0,
										}
									}
									else if let Some(email) = contact.3
									{
										ContactView::Email {
											email,
											export: contact.0,
										}
									}
									else if let Some(phone) = contact.4
									{
										ContactView::Phone {
											export: contact.0,
											phone,
										}
									}
									else
									{
										return Err(Error::Decode(
											"Row of `contact_info` did not match any `Contact` equivalent"
												.into(),
										));
									},
								);
							}
							map
						},
						status: row.get("status"),
						title: row.get("title"),
					},
					expenses: {
						let foo: Vec<(String, String, String)> = row.get("expenses");
						let foo_len = foo.len();
						foo.into_iter()
							.try_fold(
								Vec::with_capacity(foo_len),
								|mut v, (category, cost, description)| {
									v.push(Expense {
										category,
										cost: Money {
											amount: cost.parse()?,
											..Default::default()
										},
										description,
									});
									Ok(v)
								},
							)
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
					job: JobView {
						id: row.get("job_id"),
						client: OrganizationView {
							id: row.get("client_id"),
							name: row.get("client_name"),
							location: PgLocation::retrieve_view_by_id(
								connection,
								row.get("client_location_id"),
							)
							.await?,
						},
						date_close: row.get("date_close"),
						date_open: row.get("date_open"),
						increment: util::duration_from(row.get("increment"))?,
						invoice: Invoice {
							date: row
								.get::<Option<_>, _>("invoice_date_issued")
								.map(|d| InvoiceDate {
									issued: d,
									paid: row.get("invoice_date_paid"),
								}),
							hourly_rate: {
								let amount = row.get::<String, _>("invoice_hourly_rate");
								Money {
									amount: amount.parse().map_err(|e| {
										Error::Decode(
											format!(
												"`invoice_hourly_rate` is not validly formatted: {e}\n
											The constraints on table `jobs` have failed",
											)
											.into(),
										)
									})?,
									..Default::default()
								}
							},
						},
						notes: row.get("notes"),
						objectives: row.get("objectives"),
					},
					time_begin: row.get("time_begin"),
					time_end: row.get("time_end"),
					work_notes: row.get("work_notes"),
				})
			})
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
					expenses as "expenses: Vec<(String, String, String)>",
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
				.map(|(ctg, cost, description)| Expense {
					category: ctg.parse().unwrap(),
					cost: Money {
						amount: cost.parse().unwrap(),
						..Default::default()
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
