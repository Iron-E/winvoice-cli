use core::time::Duration;
use std::collections::HashMap;

use clinvoice_adapter::{
	schema::{columns::JobColumns, JobAdapter, OrganizationAdapter},
	WriteWhereClause,
};
use clinvoice_finance::{ExchangeRates, Exchangeable};
use clinvoice_match::MatchJob;
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Id,
	Invoice,
	Job,
	Organization,
};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{PgPool, QueryBuilder, Result, Row};

use super::PgJob;
use crate::{
	schema::{util, PgOrganization},
	PgSchema,
};

#[async_trait::async_trait]
impl JobAdapter for PgJob
{
	async fn create(
		connection: &PgPool,
		client: Organization,
		date_close: Option<DateTime<Utc>>,
		date_open: DateTime<Utc>,
		increment: Duration,
		invoice: Invoice,
		notes: String,
		objectives: String,
	) -> Result<Job>
	{
		let standardized_rate_fut = ExchangeRates::new()
			.map_ok(|r| invoice.hourly_rate.exchange(Default::default(), &r))
			.map_err(util::finance_err_to_sqlx);
		let standardized_rate = standardized_rate_fut.await?;

		let row = sqlx::query!(
			"INSERT INTO jobs
				(client_id, date_close, date_open, increment, invoice_date_issued, invoice_date_paid, invoice_hourly_rate, notes, objectives)
			VALUES
				($1,        $2,         $3,        $4,        $5,                  $6,                $7,                  $8,    $9)
			RETURNING id;",
			client.id,
			date_close,
			date_open,
			increment as _,
			invoice.date.as_ref().map(|d| d.issued),
			invoice.date.as_ref().and_then(|d| d.paid),
			standardized_rate.amount.to_string() as _,
			notes,
			objectives,
		)
		.fetch_one(connection)
		.await?;

		Ok(Job {
			client,
			date_close: date_close.map(util::sanitize_datetime),
			date_open: util::sanitize_datetime(date_open),
			id: row.id,
			increment,
			invoice,
			notes,
			objectives,
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: &MatchJob) -> Result<Vec<Job>>
	{
		// TODO: separate into `retrieve_all() -> Vec` and `retrieve -> Stream` to skip `Vec`
		//       collection?
		let organizations_fut =
			PgOrganization::retrieve(connection, &match_condition.client).map_ok(|vec| {
				vec.into_iter()
					.map(|o| (o.id, o))
					.collect::<HashMap<_, _>>()
			});

		let exchange_rates = ExchangeRates::new().map_err(util::finance_err_to_sqlx);

		const COLUMNS: JobColumns<&'static str> = JobColumns::default();

		let mut query = QueryBuilder::new(
			"SELECT
				J.client_id,
				J.date_close,
				J.date_open,
				J.id,
				J.increment,
				J.invoice_date_issued,
				J.invoice_date_paid,
				J.invoice_hourly_rate,
				J.notes,
				J.objectives
			FROM jobs J",
		);
		PgSchema::write_where_clause(
			Default::default(),
			"J",
			&match_condition.exchange_ref(Default::default(), &exchange_rates.await?),
			&mut query,
		);

		let organizations = organizations_fut.await?;
		query
			.push(';')
			.build()
			.fetch(connection)
			.try_filter_map(|row| {
				if let Some(o) = organizations.get(&row.get::<Id, _>(COLUMNS.client_id))
				{
					return match PgJob::row_to_view(COLUMNS, &row, o.clone())
					{
						Ok(e) => future::ok(Some(e)),
						Err(e) => future::err(e),
					};
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

	use clinvoice_adapter::schema::{LocationAdapter, OrganizationAdapter};
	use clinvoice_finance::{ExchangeRates, Exchangeable};
	use clinvoice_match::{Match, MatchInvoice, MatchJob};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		Currency,
		Invoice,
		InvoiceDate,
		Money,
	};

	use super::{JobAdapter, PgJob};
	use crate::schema::{util, PgLocation, PgOrganization};

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
		assert_eq!(job.client.id, row.client_id);
		assert_eq!(organization.id, row.client_id);
		assert_eq!(job.date_close, row.date_close);
		assert_eq!(job.date_open, row.date_open);
		assert_eq!(job.increment, util::duration_from(row.increment).unwrap());
		assert_eq!(None, row.invoice_date_issued);
		assert_eq!(None, row.invoice_date_paid);
		assert_eq!(
			job.invoice
				.hourly_rate
				.exchange(Default::default(), &ExchangeRates::new().await.unwrap()),
			Money {
				amount: row.invoice_hourly_rate.parse().unwrap(),
				..Default::default()
			},
		);
		assert_eq!(job.notes, row.notes);
		assert_eq!(job.objectives, row.objectives);
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
			PgOrganization::create(&connection, utah.clone(), "Some Other Organizatión".into()),
		)
		.unwrap();

		let (job, job2, job3, job4) = futures::try_join!(
			PgJob::create(
				&connection,
				organization.clone(),
				Some(Utc.ymd(1990, 08, 01).and_hms(09, 00, 00)),
				Utc.ymd(1990, 07, 12).and_hms(14, 10, 00),
				Duration::from_secs(300),
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
				Some(Utc.ymd(3000, 01, 16).and_hms(10, 00, 00)),
				Utc.ymd(3000, 01, 12).and_hms(09, 15, 42),
				Duration::from_secs(900),
				Invoice {
					date: Some(InvoiceDate {
						issued: Utc.ymd(3000, 01, 17).and_hms(12, 30, 00),
						paid: None,
					}),
					hourly_rate: Money::new(299_99, 2, Currency::JPY),
				},
				String::new(),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization.clone(),
				Some(Utc.ymd(2011, 03, 17).and_hms(13, 07, 07)),
				Utc.ymd(2011, 03, 17).and_hms(13, 07, 07),
				Duration::from_secs(900),
				Invoice {
					date: Some(InvoiceDate {
						issued: Utc.ymd(2011, 03, 18).and_hms(08, 00, 00),
						paid: Some(Utc.ymd(2011, 03, 19).and_hms(17, 00, 00)),
					}),
					hourly_rate: Money::new(20_00, 2, Currency::EUR),
				},
				String::new(),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization2.clone(),
				None,
				Utc.ymd(2022, 01, 02).and_hms(01, 01, 01),
				Duration::from_secs(900),
				Invoice {
					date: None,
					hourly_rate: Money::new(200_00, 2, Currency::NOK),
				},
				String::new(),
				"Do something".into()
			),
		)
		.unwrap();

		let exchange_rates = ExchangeRates::new().await.unwrap();

		assert_eq!(
			PgJob::retrieve(&connection, &MatchJob {
				id: job.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[job.exchange_ref(Default::default(), &exchange_rates)],
		);

		assert_eq!(
			PgJob::retrieve(&connection, &MatchJob {
				id: Match::Or(vec![job2.id.into(), job3.id.into()]),
				invoice: MatchInvoice {
					date_issued: Match::Not(Match::Not(Match::Any.into()).into()),
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>(),
			[
				job2.exchange(Default::default(), &exchange_rates),
				job3.exchange(Default::default(), &exchange_rates),
			]
			.into_iter()
			.collect::<HashSet<_>>(),
		);

		assert_eq!(
			PgJob::retrieve(&connection, &MatchJob {
				id: Match::Or(vec![job.id.into(), job4.id.into()]),
				invoice: MatchInvoice {
					date_issued: Match::Not(Match::Any.into()),
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>(),
			[
				job.exchange(Default::default(), &exchange_rates),
				job4.exchange(Default::default(), &exchange_rates),
			]
			.into_iter()
			.collect::<HashSet<_>>(),
		);
	}
}
