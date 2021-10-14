use clinvoice_adapter::data::Deletable;
use clinvoice_data::Person;
use sqlx::{Executor, Postgres, Result};

use super::PostgresPerson;

#[async_trait::async_trait]
impl Deletable for PostgresPerson
{
	type Db = Postgres;
	type Entity = Person;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		cascade: bool,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		todo!()
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
