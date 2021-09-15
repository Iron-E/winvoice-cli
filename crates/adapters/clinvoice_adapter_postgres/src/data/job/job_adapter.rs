use clinvoice_data::views::JobView;

use
{
	super::PostgresJob,
	crate::data::{Error, Result},

	clinvoice_adapter::data::JobAdapter,
	clinvoice_data::
	{
		chrono::{DateTime, Utc},
		Job, finance::Money, Organization
	},
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl JobAdapter for PostgresJob<'_>
{
	type Error = Error;

	async fn create(
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: String,
		pool: Self::Pool,
	) -> Result<Job>
	{
		todo!()
	}

	async fn retrieve(
		query: &query::Job,
		pool: Self::Pool,
	) -> Result<Vec<Job>>
	{
		todo!()
	}

	async fn retrieve_view(
		query: &query::Job,
		pool: Self::Pool,
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
	}

	#[tokio::test]
	async fn retrieve()
	{
	}
}
