use clinvoice_data::views::JobView;
use sqlx::{Executor, Postgres, Result};

use
{
	super::PostgresJob,

	clinvoice_adapter::data::JobAdapter,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Job, finance::Money, Organization
	},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl JobAdapter for PostgresJob
{
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
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
