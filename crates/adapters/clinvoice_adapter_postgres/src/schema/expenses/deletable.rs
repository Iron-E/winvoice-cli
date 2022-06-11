use clinvoice_adapter::Deletable;
use clinvoice_schema::{Expense, Id};
use sqlx::{Executor, Postgres, Result};

use super::PgExpenses;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgExpenses
{
	type Db = Postgres;
	type Entity = Expense;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		// TODO: use `for<'a> |x: &'a Expense| x.id`
		fn mapper(x: &Expense) -> Id
		{
			x.id
		}

		PgSchema::delete(connection, "expenses", entities.map(mapper)).await
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn delete()
	{
		// TODO: write test
	}
}
