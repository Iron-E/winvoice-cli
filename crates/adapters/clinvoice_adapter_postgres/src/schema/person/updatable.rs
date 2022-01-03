use clinvoice_adapter::Updatable;
use clinvoice_schema::Person;
use sqlx::{Executor, Postgres, Result};

use super::PgPerson;

#[async_trait::async_trait]
impl Updatable for PgPerson
{
	type Db = Postgres;
	type Entity = Person;

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
