use clinvoice_adapter::Updatable;
use clinvoice_schema::Expense;
use sqlx::{Postgres, Result, Transaction};

use super::PgExpenses;

#[async_trait::async_trait]
impl Updatable for PgExpenses
{
	type Db = Postgres;
	type Entity = Expense;

	async fn update<'e>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'e Self::Entity> + Send,
	) -> Result<()>
	where
		Self::Entity: 'e,
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
