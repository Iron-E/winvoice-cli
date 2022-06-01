use clinvoice_adapter::Deletable;
use clinvoice_schema::Expense;
use sqlx::{Executor, Postgres, Result};

use super::PgExpenses;

#[async_trait::async_trait]
impl Deletable for PgExpenses
{
	type Db = Postgres;
	type Entity = Expense;

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
