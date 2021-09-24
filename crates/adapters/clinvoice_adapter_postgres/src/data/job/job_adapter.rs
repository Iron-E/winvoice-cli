use core::time::Duration;

use clinvoice_adapter::data::JobAdapter;
use clinvoice_data::{
	chrono::{DateTime, Utc},
	views::JobView,
	Job,
	Money,
	Organization,
};
use clinvoice_query as query;
use sqlx::{Executor, Postgres, Result};

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
		todo!()
	}

	async fn retrieve(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Job,
	) -> Result<Vec<Job>>
	{
		todo!()
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
