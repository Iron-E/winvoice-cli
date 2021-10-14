use clinvoice_adapter::data::Updatable;
use clinvoice_data::Location;
use sqlx::{Executor, Postgres, Result};

use super::PostgresLocation;

#[async_trait::async_trait]
impl Updatable for PostgresLocation
{
	type Db = Postgres;
	type Entity = Location;

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
