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
