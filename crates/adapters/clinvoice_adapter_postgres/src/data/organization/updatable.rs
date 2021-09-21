use clinvoice_adapter::data::Updatable;
use clinvoice_data::Organization;
use sqlx::{Error, Executor, Postgres, Result};

use super::PostgresOrganization;

#[async_trait::async_trait]
impl Updatable for PostgresOrganization
{
	type Db = Postgres;
	type Entity = Organization;
	type Error = Error;

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
