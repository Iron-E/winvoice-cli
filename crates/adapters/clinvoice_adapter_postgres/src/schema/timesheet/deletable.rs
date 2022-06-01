use clinvoice_adapter::Deletable;
use clinvoice_schema::Timesheet;
use sqlx::{Executor, Postgres, Result};

use super::PgTimesheet;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgTimesheet
{
	type Db = Postgres;
	type Entity = Timesheet;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		PgSchema::delete(connection, "timesheets", entities.map(|e| e.id)).await
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
