use clinvoice_adapter::Deletable;
use clinvoice_schema::{Employee, Id};
use sqlx::{Executor, Postgres, Result};

use super::PgEmployee;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgEmployee
{
	type Db = Postgres;
	type Entity = Employee;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		// TODO: use `for<'a> |e: &'a Employee| e.id`
		fn mapper(e: &Employee) -> Id
		{
			e.id
		}

		PgSchema::delete(connection, "employees", entities.map(mapper)).await
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
