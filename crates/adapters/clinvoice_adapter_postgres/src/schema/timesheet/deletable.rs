use clinvoice_adapter::Deletable;
use clinvoice_schema::Timesheet;
use sqlx::{Acquire, Postgres, Result};

use super::PostgresTimesheet;

#[async_trait::async_trait]
impl Deletable for PostgresTimesheet
{
	type Db = Postgres;
	type Entity = Timesheet;

	async fn delete(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
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
