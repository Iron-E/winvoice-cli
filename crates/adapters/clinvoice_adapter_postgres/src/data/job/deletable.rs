use clinvoice_data::Job;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresJob,

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl Deletable for PostgresJob
{
	type Db = Postgres;
	type Entity = Job;
	type Error = Error;

	async fn delete(cascade: bool, connection: impl Executor<'_, Database = Self::Db>, entities: &[Self::Entity]) -> Result<()>
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
