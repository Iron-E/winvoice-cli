use core::time::Duration;
use std::convert::TryFrom;

use clinvoice_adapter::data::JobAdapter;
use clinvoice_data::{
	chrono::{DateTime, Utc},
	views::JobView,
	Invoice,
	Job,
	Money,
	Organization,
};
use clinvoice_query as query;
use sqlx::{
	postgres::{
		types::{PgInterval, PgMoney},
		Postgres,
	},
	Executor,
	Result,
};

use super::PostgresJob;

#[async_trait::async_trait]
impl JobAdapter for PostgresJob
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		increment: Duration,
		objectives: String,
	) -> Result<Job>
	{
		// TODO: use `decimal` instead of `money` for monetary type
		let pg_increment = PgInterval::try_from(increment).map_err(|e| sqlx::Error::Decode(e))?;
		// let pg_hourly_rate_amount = PgMoney::from_decimal(hourly_rate.amount);
		let row = sqlx::query!(
			"INSERT INTO jobs
				(client_id, date_close, date_open, increment, invoice,                      objectives)
			VALUES
				($1,        NULL,       $2,        $3,        ROW(NULL, NULL, ROW($4, $5)), $6)
			RETURNING id;",
			client.id,
			date_open,
			pg_increment,
			pg_hourly_rate_amount,
			hourly_rate.currency.as_str() as _,
			objectives,
		)
		.fetch_one(connection)
		.await?;

		Ok(Job {
			client_id: client.id,
			date_close: None,
			date_open,
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
	#[tokio::test]
	async fn create()
	{
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve()
	{
		// TODO: write test
	}

	#[tokio::test]
	async fn retrieve_view()
	{
		// TODO: write test
	}
}
