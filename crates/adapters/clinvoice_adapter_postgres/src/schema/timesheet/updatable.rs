use clinvoice_adapter::Updatable;
use clinvoice_schema::Timesheet;
use sqlx::{Executor, Postgres, Result};

use super::PgTimesheet;

#[async_trait::async_trait]
impl Updatable for PgTimesheet
{
	type Db = Postgres;
	type Entity = Timesheet;

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
