use sqlx::{Postgres, Executor, Error, Result};
use
{
	super::PostgresLocation,

	clinvoice_adapter::data::Deletable,
	clinvoice_data::Location,
};

#[async_trait::async_trait]
impl Deletable for PostgresLocation
{
	type Db = Postgres;
	type Entity = Location;
	type Error = Error;

	async fn delete(cascade: bool, connection: impl 'async_trait + Executor<'_, Database = Self::Db>, entities: &[Self::Entity]) -> Result<()>
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
