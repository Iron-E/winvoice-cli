use core::time::Duration;
use std::convert::TryFrom;

use clinvoice_adapter::{schema::JobAdapter, WriteWhereClause};
use clinvoice_finance::{Currency, Error as FinanceError, ExchangeRates};
use clinvoice_match::{MatchInvoice, MatchJob};
use clinvoice_schema::{
	chrono::{DateTime, SubsecRound, Utc},
	views::{JobView, OrganizationView},
	Invoice,
	InvoiceDate,
	Job,
	Money,
	Organization,
};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{postgres::types::PgInterval, Error, PgPool, Result, Row};

use super::PgJob;
use crate::{
	schema::{util, PgLocation},
	PgSchema as Schema,
};

#[async_trait::async_trait]
impl JobAdapter for PgJob
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
		let standardized_rate_fut = ExchangeRates::new()
			.map_ok(|r| hourly_rate.exchange(Default::default(), &r))
			.map_err(|e| match e
			{
				FinanceError::Decimal(e2) => Error::Decode(e2.into()),
				FinanceError::Io(e2) => Error::Io(e2),
				FinanceError::Reqwest(e2) => Error::Protocol(e2.to_string()),
				FinanceError::UnsupportedCurrency(_) => Error::Decode(e.into()),
			});
		let pg_increment = PgInterval::try_from(increment).map_err(Error::Decode)?;
		let standardized_rate = standardized_rate_fut.await?;

		let row = sqlx::query!(
			"INSERT INTO jobs
				(client_id, date_close, date_open, increment, invoice_date_issued, invoice_date_paid, invoice_hourly_rate, notes, objectives)
			VALUES
				($1,        NULL,       $2,        $3,        NULL,                NULL,              $4,                  '',    $5)
			RETURNING id;",
			client.id,
			date_open,
			pg_increment,
			standardized_rate.amount.to_string() as _,
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
				hourly_rate: standardized_rate,
			},
			notes: String::new(),
			objectives,
		})
	}

	async fn retrieve_view(connection: &PgPool, match_condition: &MatchJob) -> Result<Vec<JobView>>
	{
		let exchange_rates = ExchangeRates::new().map_err(|e| match e
		{
			FinanceError::Decimal(e2) => Error::Decode(e2.into()),
			FinanceError::Io(e2) => Error::Io(e2),
			FinanceError::Reqwest(e2) => Error::Protocol(e2.to_string()),
			FinanceError::UnsupportedCurrency(_) => Error::Decode(e.into()),
		});

		let id_match =
			PgLocation::retrieve_matching_ids(connection, &match_condition.client.location);
		let mut query = String::from(
			"SELECT
				J.id, J.client_id, J.date_close, J.date_open, J.increment, J.invoice_date_issued, \
			 J.invoice_date_paid, J.invoice_hourly_rate, J.notes, J.objectives,
				O.name, O.location_id
			FROM jobs J
			JOIN organizations O ON (O.id = J.client_id)",
		);
		Schema::write_where_clause(
			Schema::write_where_clause(
				Schema::write_where_clause(
					Default::default(),
					"J",
					// PERF: there must be something to do to remove all of these clones
					&MatchJob {
						client: match_condition.client.clone(),
						date_close: match_condition.date_close.clone(),
						date_open: match_condition.date_open.clone(),
						id: match_condition.id.clone(),
						increment: match_condition.increment.clone(),
						invoice: MatchInvoice {
							date_issued: match_condition.invoice.date_issued.clone(),
							date_paid: match_condition.invoice.date_paid.clone(),
							hourly_rate: match_condition
								.invoice
								.hourly_rate
								.clone()
								.exchange(Currency::EUR, &exchange_rates.await?),
						},
						notes: match_condition.notes.clone(),
						objectives: match_condition.objectives.clone(),
					},
					&mut query,
				),
				"O",
				&match_condition.client,
				&mut query,
			),
			"O.location_id",
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
						location: PgLocation::retrieve_view_by_id(connection, row.get("location_id"))
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
											"`invoice_hourly_rate` is not validly formatted: {}\n
										The constraints on table `jobs` have failed",
											e
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

	use clinvoice_adapter::schema::{LocationAdapter, OrganizationAdapter};
	use clinvoice_finance::ExchangeRates;
	use clinvoice_match::MatchJob;
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		views::{JobView, LocationView, OrganizationView},
		Currency,
		Money,
	};

	use super::{JobAdapter, PgJob};
	use crate::schema::{util, PgLocation, PgOrganization};

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

		let row = sqlx::query!(
			r#"SELECT
					id,
					client_id,
					date_close,
					date_open,
					increment,
					invoice_date_issued,
					invoice_date_paid,
					invoice_hourly_rate,
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
			job.invoice.hourly_rate,
			Money {
				amount: row.invoice_hourly_rate.parse().unwrap(),
				..Default::default()
			}
			.exchange(
				job.invoice.hourly_rate.currency,
				&ExchangeRates::new().await.unwrap()
			),
		);
		assert_eq!(job.notes, row.notes);
		assert_eq!(job.objectives, row.objectives);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into())
			.await
			.unwrap();
		let usa = PgLocation::create_inner(&connection, &earth, "USA".into())
			.await
			.unwrap();
		let (arizona, utah) = futures::try_join!(
			PgLocation::create_inner(&connection, &usa, "Arizona".into()),
			PgLocation::create_inner(&connection, &usa, "Utah".into()),
		)
		.unwrap();

		let (some_organization, some_other_organization) = futures::try_join!(
			PgOrganization::create(&connection, &arizona, "Some Organization".into()),
			PgOrganization::create(&connection, &utah, "Some Other Organizatión".into()),
		)
		.unwrap();

		let (job_one, job_two, job_three, job_four) = futures::try_join!(
			PgJob::create(
				&connection,
				&some_organization,
				Utc.ymd(1990, 07, 12).and_hms(14, 10, 00),
				Money::new(20_00, 2, Currency::USD),
				Duration::from_secs(900),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				&some_other_organization,
				Utc.ymd(3000, 01, 12).and_hms(09, 15, 42),
				Money::new(200_00, 2, Currency::JPY),
				Duration::from_secs(900),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				&some_organization,
				Utc.ymd(2011, 03, 17).and_hms(13, 07, 07),
				Money::new(20_00, 2, Currency::EUR),
				Duration::from_secs(900),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				&some_other_organization,
				Utc.ymd(2022, 01, 02).and_hms(01, 01, 01),
				Money::new(200_00, 2, Currency::NOK),
				Duration::from_secs(900),
				"Do something".into()
			),
		)
		.unwrap();

		let earth_view = LocationView {
			id: earth.id,
			name: earth.name.clone(),
			outer: None,
		};
		let usa_view = LocationView {
			id: usa.id,
			name: usa.name.clone(),
			outer: Some(earth_view.clone().into()),
		};
		let arizona_view = LocationView {
			id: arizona.id,
			name: arizona.name.clone(),
			outer: Some(usa_view.clone().into()),
		};
		let utah_view = LocationView {
			id: utah.id,
			name: utah.name.clone(),
			outer: Some(usa_view.clone().into()),
		};

		let some_organization_view = OrganizationView {
			id: some_organization.id,
			name: some_organization.name.clone(),
			location: arizona_view,
		};
		let some_other_organization_view = OrganizationView {
			id: some_other_organization.id,
			name: some_other_organization.name.clone(),
			location: utah_view,
		};

		let job_one_view = JobView {
			client: some_organization_view.clone(),
			date_close: job_one.date_close,
			date_open: job_one.date_open,
			id: job_one.id,
			increment: job_one.increment,
			invoice: job_one.invoice,
			notes: job_one.notes,
			objectives: job_one.objectives,
		};
		let job_two_view = JobView {
			client: some_other_organization_view.clone(),
			date_close: job_two.date_close,
			date_open: job_two.date_open,
			id: job_two.id,
			increment: job_two.increment,
			invoice: job_two.invoice,
			notes: job_two.notes,
			objectives: job_two.objectives,
		};
		let job_three_view = JobView {
			client: some_organization_view,
			date_close: job_three.date_close,
			date_open: job_three.date_open,
			id: job_three.id,
			increment: job_three.increment,
			invoice: job_three.invoice,
			notes: job_three.notes,
			objectives: job_three.objectives,
		};
		let job_four_view = JobView {
			client: some_other_organization_view,
			date_close: job_four.date_close,
			date_open: job_four.date_open,
			id: job_four.id,
			increment: job_four.increment,
			invoice: job_four.invoice,
			notes: job_four.notes,
			objectives: job_four.objectives,
		};

		assert_eq!(
			PgJob::retrieve_view(&connection, &MatchJob {
				id: job_one.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[job_one_view.clone()],
		);
	}
}
