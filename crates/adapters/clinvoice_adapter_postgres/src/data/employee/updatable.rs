use clinvoice_adapter::data::Updatable;
use clinvoice_data::Employee;
use sqlx::{Executor, Postgres, Result};

use super::PostgresEmployee;

#[async_trait::async_trait]
impl Updatable for PostgresEmployee
{
	type Db = Postgres;
	type Entity = Employee;

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
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn update()
	{
		// TODO: write test
	}
}
