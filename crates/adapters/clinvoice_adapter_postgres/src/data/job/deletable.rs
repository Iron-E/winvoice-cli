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

	async fn delete(connection: impl 'async_trait + Executor<'_, Database = Self::Db>, cascade: bool, entities: &[Self::Entity]) -> Result<()>
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
