use clinvoice_adapter::Deletable;
use clinvoice_schema::Location;
use sqlx::{Acquire, Postgres, Result};

use super::PostgresLocation;

#[async_trait::async_trait]
impl Deletable for PostgresLocation
{
	type Db = Postgres;
	type Entity = Location;

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
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn delete()
	{
		// TODO: write test
	}
}
