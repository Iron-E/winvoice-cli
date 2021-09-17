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
