use clinvoice_adapter::Deletable;
use clinvoice_schema::Organization;
use sqlx::{Executor, Postgres, Result};

use super::PgOrganization;

#[async_trait::async_trait]
impl Deletable for PgOrganization
{
	type Db = Postgres;
	type Entity = Organization;

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
