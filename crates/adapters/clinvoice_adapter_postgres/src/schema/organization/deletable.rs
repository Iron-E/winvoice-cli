use clinvoice_adapter::Deletable;
use clinvoice_schema::Organization;
use sqlx::{Executor, Postgres, Result};

use super::PgOrganization;
use crate::PgSchema;

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
		PgSchema::delete(connection, "organization", entities.map(|e| e.id)).await
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
