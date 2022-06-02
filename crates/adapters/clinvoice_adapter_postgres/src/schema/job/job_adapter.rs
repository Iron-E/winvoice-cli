use core::time::Duration;
use std::{collections::HashMap, convert::TryFrom};

use clinvoice_adapter::{
	schema::{JobAdapter, OrganizationAdapter},
	WriteWhereClause,
};
use clinvoice_finance::ExchangeRates;
use clinvoice_match::{MatchInvoice, MatchJob};
use clinvoice_schema::{
	chrono::{DateTime, SubsecRound, Utc},
	Id,
	Invoice,
	Job,
	Money,
	Organization,
};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{postgres::types::PgInterval, Error, PgPool, QueryBuilder, Result, Row};

use super::PgJob;
use crate::{
	schema::{job::columns::PgJobColumns, util, PgOrganization},
	PgSchema,
};

#[async_trait::async_trait]
impl JobAdapter for PgJob
{
	async fn create(
		connection: &PgPool,
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		increment: Duration,
		objectives: String,
	) -> Result<Job>
	{
		let standardized_rate_fut = ExchangeRates::new()
			.map_ok(|r| hourly_rate.exchange(Default::default(), &r))
			.map_err(util::finance_err_to_sqlx);
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
			client,
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

	async fn retrieve(connection: &PgPool, match_condition: MatchJob) -> Result<Vec<Job>>
	{
		// TODO: separate into `retrieve_all() -> Vec` and `retrieve -> Stream` to skip `Vec`
		//       collection?
		let organizations_fut = PgOrganization::retrieve(connection, match_condition.client.clone())
			.map_ok(|vec| {
				vec.into_iter()
					.map(|o| (o.id, o))
					.collect::<HashMap<_, _>>()
			});

		let exchange_rates = ExchangeRates::new().map_err(util::finance_err_to_sqlx);

		const COLUMNS: PgJobColumns<&'static str> = PgJobColumns::new();

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
			&MatchJob {
				client: match_condition.client,
				date_close: match_condition.date_close,
				date_open: match_condition.date_open,
				id: match_condition.id,
				increment: match_condition.increment,
				invoice: MatchInvoice {
					date_issued: match_condition.invoice.date_issued,
					date_paid: match_condition.invoice.date_paid,
					hourly_rate: match_condition
						.invoice
						.hourly_rate
						.exchange(Default::default(), &exchange_rates.await?),
				},
				notes: match_condition.notes,
				objectives: match_condition.objectives,
			},
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
					return match COLUMNS.row_to_view(o.clone(), &row)
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
	use clinvoice_finance::ExchangeRates;
	use clinvoice_match::{Match, MatchJob, MatchLocation, MatchOrganization};
	use clinvoice_schema::{
		chrono::{TimeZone, Utc},
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

		let organization =
			PgOrganization::create(&connection, Vec::new(), earth, "Some Organization".into())
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
				Vec::new(),
				arizona.clone(),
				"Some Organization".into()
			),
			PgOrganization::create(
				&connection,
				Vec::new(),
				utah.clone(),
				"Some Other Organizatión".into()
			),
		)
		.unwrap();

		let (job, job2, job3, job4) = futures::try_join!(
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
			PgJob::create(
				&connection,
				organization.clone(),
				Utc.ymd(2011, 03, 17).and_hms(13, 07, 07),
				Money::new(20_00, 2, Currency::EUR),
				Duration::from_secs(900),
				"Do something".into()
			),
			PgJob::create(
				&connection,
				organization2.clone(),
				Utc.ymd(2022, 01, 02).and_hms(01, 01, 01),
				Money::new(200_00, 2, Currency::NOK),
				Duration::from_secs(900),
				"Do something".into()
			),
		)
		.unwrap();

		assert_eq!(
			PgJob::retrieve(&connection, MatchJob {
				id: job.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[job.clone()],
		);

		assert_eq!(
			PgJob::retrieve(&connection, MatchJob {
				client: MatchOrganization {
					location: MatchLocation {
						id: Match::Or(vec![
							organization.location.id.into(),
							organization2.location.id.into()
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
			[job.clone(), job2.clone(), job3.clone(), job4.clone(),]
				.into_iter()
				.collect::<HashSet<_>>(),
		);
	}
}
