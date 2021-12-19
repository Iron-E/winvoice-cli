use core::time::Duration;
use std::convert::TryFrom;

use clinvoice_adapter::{schema::JobAdapter, WriteWhereClause};
use clinvoice_match::MatchJob;
use clinvoice_schema::{
	chrono::{DateTime, SubsecRound, Utc},
	views::{JobView, OrganizationView},
	Invoice,
	InvoiceDate,
	Job,
	Money,
	Organization,
};
use futures::TryStreamExt;
use sqlx::{postgres::types::PgInterval, Error, PgPool, Result, Row};

use super::PostgresJob;
use crate::{
	schema::{util, PostgresLocation},
	PostgresSchema as Schema,
};

#[async_trait::async_trait]
impl JobAdapter for PostgresJob
{
	async fn create(
		connection: &PgPool,
		client: &Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		increment: Duration,
		objectives: String,
	) -> Result<Job>
	{
		let pg_increment = PgInterval::try_from(increment).map_err(Error::Decode)?;
		let row = sqlx::query!(
			"INSERT INTO jobs
				(client_id, date_close, date_open, increment, invoice_date_issued, invoice_date_paid, invoice_hourly_rate, notes, objectives)
			VALUES
				($1,        NULL,       $2,        $3,        NULL,                NULL,              ROW($4, $5),         '',    $6)
			RETURNING id;",
			client.id,
			date_open,
			pg_increment,
			hourly_rate.amount.to_string(),
			hourly_rate.currency.as_str() as _,
			objectives,
		)
		.fetch_one(connection)
		.await?;

		Ok(Job {
			client_id: client.id,
			date_close: None,
			date_open: date_open.trunc_subsecs(6),
			id: row.id,
			increment,
			invoice: Invoice {
				date: None,
				hourly_rate,
			},
			notes: String::new(),
			objectives,
		})
	}

	async fn retrieve_view(connection: &PgPool, match_condition: &MatchJob) -> Result<Vec<JobView>>
	{
		let id_match =
			PostgresLocation::retrieve_matching_ids(connection, &match_condition.client.location);
		let mut query = String::from(
			"SELECT
				J.id, J.client_id, J.date_close, J.date_open, J.increment, J.invoice_date_issued, \
			 J.invoice_date_paid, J.invoice_hourly_rate, J.notes, J.objectives,
				O.name, O.location_id,
				P.name
			FROM jobs J
			JOIN organizations O ON (O.id = J.client_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(Default::default(), "J", match_condition, &mut query),
				"O",
				&match_condition.client,
				&mut query,
			),
			"L.id",
			&id_match.await?,
			&mut query,
		);
		query.push(';');

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| async move {
				Ok(JobView {
					id: row.get("id"),
					client: OrganizationView {
						id: row.get("client_id"),
						name: row.get("name"),
						location: PostgresLocation::retrieve_view_by_id(
							connection,
							row.get("location_id"),
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
							fn map_err(e: impl std::error::Error) -> Error
							{
								Error::Decode(
									format!(
										"`invoice_hourly_rate` is not validly formatted: {}\n
										The constraints on table `jobs` have failed",
										e
									)
									.into(),
								)
							}

							let (amount, currency) = row.get::<(String, String), _>("invoice_hourly_rate");
							Money {
								amount: amount.parse().map_err(map_err)?,
								currency: currency.parse().map_err(map_err)?,
							}
						},
					},
					notes: row.get("notes"),
					objectives: row.get("objectives"),
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
		LocationAdapter,
		OrganizationAdapter,
		PersonAdapter,
	};
	use clinvoice_schema::{chrono::Utc, Contact, Currency, Money};

	use super::{JobAdapter, PostgresJob};
	use crate::schema::{
		util,
		PostgresEmployee,
		PostgresLocation,
		PostgresOrganization,
		PostgresPerson,
	};

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PostgresLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let person = PostgresPerson::create(&connection, "My Name".into())
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

		let employee = PostgresEmployee::create(
			&connection,
			contact_info,
			&organization,
			&person,
			"Employed".into(),
			"Janitor".into(),
		)
		.await
		.unwrap();

		let job = PostgresJob::create(
			&connection,
			&organization,
			Utc::now(),
			Money::new(13_27, 2, Currency::USD),
			Duration::new(7640, 0),
			"Write the test".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!(
			r#"SELECT
					id,
					client_id,
					date_close,
					date_open,
					increment,
					invoice_date_issued,
					invoice_date_paid,
					(invoice_hourly_rate).amount as invoice_hourly_rate_amount,
					(invoice_hourly_rate).currency as "invoice_hourly_rate_currency: String",
					notes,
					objectives
				FROM jobs
				WHERE id = $1;"#,
			job.id,
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(job.id, row.id);
		assert_eq!(job.client_id, row.client_id);
		assert_eq!(organization.id, row.client_id);
		assert_eq!(job.date_close, row.date_close);
		assert_eq!(job.date_open, row.date_open);
		assert_eq!(job.increment, util::duration_from(row.increment).unwrap());
		assert_eq!(None, row.invoice_date_issued);
		assert_eq!(None, row.invoice_date_paid);
		assert_eq!(
			job.invoice.hourly_rate.amount,
			row.invoice_hourly_rate_amount.unwrap().parse().unwrap()
		);
		assert_eq!(
			job.invoice.hourly_rate.currency,
			row.invoice_hourly_rate_currency.unwrap().parse().unwrap()
		);
		assert_eq!(job.notes, row.notes);
		assert_eq!(job.objectives, row.objectives);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
