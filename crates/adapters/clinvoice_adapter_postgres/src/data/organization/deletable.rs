use clinvoice_data::Organization;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresOrganization,

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl Deletable for PostgresOrganization
{
	type Db = Postgres;
	type Entity = Organization;
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
