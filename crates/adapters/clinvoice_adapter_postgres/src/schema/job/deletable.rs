use clinvoice_adapter::Deletable;
use clinvoice_schema::Job;
use sqlx::{Executor, Postgres, Result};

use super::PostgresJob;

#[async_trait::async_trait]
impl Deletable for PostgresJob
{
	type Db = Postgres;
	type Entity = Job;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		cascade: bool,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn delete()
	{
		// TODO: write test
	}
}