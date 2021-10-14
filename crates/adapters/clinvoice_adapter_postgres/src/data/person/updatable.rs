use clinvoice_adapter::data::Updatable;
use clinvoice_data::Person;
use sqlx::{Executor, Postgres, Result};

use super::PostgresPerson;

#[async_trait::async_trait]
impl Updatable for PostgresPerson
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
	#[tokio::test]
	async fn update()
	{
		// TODO: write test
	}
}
