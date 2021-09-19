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
