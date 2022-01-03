use clinvoice_adapter::Updatable;
use clinvoice_schema::Job;
use sqlx::{Executor, Postgres, Result};

use super::PgJob;

#[async_trait::async_trait]
impl Updatable for PgJob
{
	type Db = Postgres;
	type Entity = Job;

	async fn update(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entity: Self::Entity,
	) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn update()
	{
		// TODO: write test
	}
}
