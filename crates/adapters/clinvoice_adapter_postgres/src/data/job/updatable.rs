use clinvoice_adapter::data::Updatable;
use clinvoice_data::Job;
use sqlx::{Executor, Postgres, Result};

use super::PostgresJob;

#[async_trait::async_trait]
impl Updatable for PostgresJob
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
	#[tokio::test]
	async fn update()
	{
		// TODO: write test
	}
}
