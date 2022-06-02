use clinvoice_adapter::Updatable;
use clinvoice_schema::Expense;
use sqlx::{Postgres, Result, Transaction};

use super::PgExpenses;

#[async_trait::async_trait]
impl Updatable for PgExpenses
{
	type Db = Postgres;
	type Entity = Expense;

	async fn update(connection: &mut Transaction<Self::Db>, entity: Self::Entity) -> Result<()>
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
