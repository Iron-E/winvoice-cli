use core::time::Duration;

use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, TableToSql},
	schema::{
		columns::{JobColumns, LocationColumns, OrganizationColumns},
		JobAdapter,
	},
	WriteWhereClause,
};
use clinvoice_finance::{ExchangeRates, Exchangeable};
use clinvoice_match::MatchJob;
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Invoice,
	Job,
	Organization,
};
use futures::{TryFutureExt, TryStreamExt};
use sqlx::{PgPool, Result};

use super::PgJob;
use crate::{
	fmt::{DateTimeExt, PgLocationRecursiveCte},
	schema::{util, PgLocation},
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
		let standardized_rate = ExchangeRates::new()
			.await
			.map(|r| invoice.hourly_rate.exchange(Default::default(), &r))
			.map_err(util::finance_err_to_sqlx)?;

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
			date_close,
			date_open,
			id: row.id,
			increment,
			invoice,
			notes,
			objectives,
		}
		.pg_sanitize())
	}

	async fn retrieve(connection: &PgPool, match_condition: &MatchJob) -> Result<Vec<Job>>
	{
		const COLUMNS: JobColumns<&str> = JobColumns::default();

		const ORGANIZATION_COLUMNS_UNIQUE: OrganizationColumns<&str> = OrganizationColumns::unique();

		let columns = COLUMNS.default_scope();
		let exchange_rates_fut = ExchangeRates::new().map_err(util::finance_err_to_sqlx);
		let mut query = PgLocation::query_with_recursive(&match_condition.client.location);
		let organization_columns = OrganizationColumns::default().default_scope();

		query
			.push(sql::SELECT)
			.push_columns(&columns)
			.push_more_columns(&organization_columns.r#as(ORGANIZATION_COLUMNS_UNIQUE))
			.push_default_from::<JobColumns<char>>()
			.push_default_equijoin::<OrganizationColumns<char>, _, _>(
				organization_columns.id,
				columns.client_id,
			)
			.push_equijoin(
				PgLocationRecursiveCte::from(&match_condition.client.location),
				LocationColumns::<char>::DEFAULT_ALIAS,
				LocationColumns::default().default_scope().id,
				organization_columns.location_id,
			);

		PgSchema::write_where_clause(
			PgSchema::write_where_clause(
				Default::default(),
				JobColumns::<char>::DEFAULT_ALIAS,
				&match_condition.exchange_ref(Default::default(), &exchange_rates_fut.await?),
				&mut query,
			),
			OrganizationColumns::<char>::DEFAULT_ALIAS,
			&match_condition.client,
			&mut query,
		);

		query
			.prepare()
			.fetch(connection)
			.and_then(|row| async move {
				PgJob::row_to_view(connection, COLUMNS, ORGANIZATION_COLUMNS_UNIQUE, &row).await
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
	use clinvoice_match::{Match, MatchInvoice, MatchJob, MatchOption};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
		Currency,
		Invoice,
		InvoiceDate,
		Money,
	};
	use pretty_assertions::assert_eq;

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
				hourly_rate: Money::new(13_27, 2, Currency::Usd),
			},
			String::new(),
			"Write the test".into(),
		)
		.await
		.unwrap();

		let row = sqlx::query!(
			"SELECT
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
				WHERE id = $1;",
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
					hourly_rate: Money::new(20_00, 2, Currency::Usd),
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
					hourly_rate: Money::new(299_99, 2, Currency::Jpy),
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
					hourly_rate: Money::new(20_00, 2, Currency::Eur),
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
					hourly_rate: Money::new(200_00, 2, Currency::Nok),
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
					date_issued: MatchOption::Not(Box::new(None.into())),
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
					date_issued: None.into(),
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
