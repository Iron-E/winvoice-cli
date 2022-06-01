use clinvoice_adapter::Deletable;
use clinvoice_schema::Employee;
use sqlx::{Executor, Postgres, Result};

use super::PgEmployee;

#[async_trait::async_trait]
impl Deletable for PgEmployee
{
	type Db = Postgres;
	type Entity = Employee;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn delete()
	{
		// TODO: write test
	}
}
