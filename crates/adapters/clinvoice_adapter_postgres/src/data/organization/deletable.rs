use clinvoice_adapter::data::Deletable;
use clinvoice_data::Organization;
use sqlx::{Error, Executor, Postgres, Result};

use super::PostgresOrganization;

#[async_trait::async_trait]
impl Deletable for PostgresOrganization
{
	type Db = Postgres;
	type Entity = Organization;
	type Error = Error;

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
