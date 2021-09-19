use clinvoice_data::Employee;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresEmployee,

	clinvoice_adapter::data::Deletable,
};

#[async_trait::async_trait]
impl Deletable for PostgresEmployee
{
	type Db = Postgres;
	type Entity = Employee;
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
