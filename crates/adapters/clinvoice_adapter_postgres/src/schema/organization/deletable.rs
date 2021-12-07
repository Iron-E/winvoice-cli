use clinvoice_adapter::Deletable;
use clinvoice_schema::Organization;
use sqlx::{Acquire, Postgres, Result};

use super::PostgresOrganization;

#[async_trait::async_trait]
impl Deletable for PostgresOrganization
{
	type Db = Postgres;
	type Entity = Organization;

	async fn delete(
		connection: impl 'async_trait + Acquire<'_, Database = Self::Db> + Send,
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
	#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
	async fn delete()
	{
		// TODO: write test
	}
}
