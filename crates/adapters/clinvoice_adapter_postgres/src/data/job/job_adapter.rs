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
use futures::Stream;
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

	fn retrieve<'a, S>(connection: impl Executor<'a, Database = Postgres>, query: &query::Job) -> S
	where
		S: Stream<Item = Result<Job>>,
	{
		todo!()
	}

	fn retrieve_view<'a, S>(
		connection: impl Executor<'a, Database = Postgres>,
		query: &query::Job,
	) -> S
	where
		S: Stream<Item = Result<JobView>>,
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
