use clinvoice_data::Location;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresLocation,

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresLocation
{
	type Db = Postgres;
	type Entity = Location;
	type Error = Error;

	async fn update(entity: &Self::Entity, connection: impl Executor<'_, Database = Self::Db>) -> Result<()>
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
