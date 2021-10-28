use core::time::Duration;
use std::convert::TryFrom;

use clinvoice_adapter::data::JobAdapter;
use clinvoice_data::{
	chrono::{DateTime, SubsecRound, Utc},
	views::JobView,
	Invoice,
	Job,
	Money,
	Organization,
};
use clinvoice_query as query;
use sqlx::{
	postgres::{types::PgInterval, Postgres},
	Error,
	Executor,
	Result,
};

use super::PostgresJob;

#[async_trait::async_trait]
impl JobAdapter for PostgresJob
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
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

	async fn retrieve_view(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Job,
	) -> Result<Vec<JobView>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use clinvoice_adapter::data::{Initializable, LocationAdapter, OrganizationAdapter};
	use clinvoice_data::{chrono::Utc, Currency, Money};

	use super::{JobAdapter, PostgresJob};
	use crate::data::{util, PostgresLocation, PostgresOrganization, PostgresSchema};

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn create()
	{
		let mut connection = util::connect().await;

		PostgresSchema::init(&mut connection).await.unwrap();

		let earth = PostgresLocation::create(&mut connection, "Earth".into())
			.await
			.unwrap();

		let organization =
			PostgresOrganization::create(&mut connection, &earth, "Some Organization".into())
				.await
				.unwrap();

		let job = PostgresJob::create(
			&mut connection,
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
				FROM jobs;"#
		)
		.fetch_one(&mut connection)
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

	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
