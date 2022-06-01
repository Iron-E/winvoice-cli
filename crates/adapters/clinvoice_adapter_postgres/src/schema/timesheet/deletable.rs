use clinvoice_adapter::Deletable;
use clinvoice_schema::Timesheet;
use sqlx::{Executor, Postgres, Result};

use super::PgTimesheet;

#[async_trait::async_trait]
impl Deletable for PgTimesheet
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
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn delete()
	{
		// TODO: write test
	}
}
