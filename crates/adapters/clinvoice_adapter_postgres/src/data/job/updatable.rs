use clinvoice_data::Job;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresJob,

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresJob
{
	type Db = Postgres;
	type Entity = Job;
	type Error = Error;

	async fn update(connection: impl 'async_trait + Executor<'_, Database = Self::Db>, entity: &Self::Entity) -> Result<()>
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
