use clinvoice_adapter::Deletable;
use clinvoice_schema::{Id, Location};
use sqlx::{Executor, Postgres, Result};

use super::PgLocation;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgLocation
{
	type Db = Postgres;
	type Entity = Location;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		// TODO: use `for<'a> |l: &'a Location| l.id`
		fn mapper(l: &Location) -> Id
		{
			l.id
		}

		PgSchema::delete(connection, "locations", entities.map(mapper)).await
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
