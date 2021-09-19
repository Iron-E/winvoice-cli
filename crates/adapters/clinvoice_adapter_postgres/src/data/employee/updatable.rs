use clinvoice_data::Employee;
use sqlx::{Postgres, Executor, Error, Result};

use
{
	super::PostgresEmployee,

	clinvoice_adapter::data::Updatable,
};

#[async_trait::async_trait]
impl Updatable for PostgresEmployee
{
	type Db = Postgres;
	type Entity = Employee;
	type Error = Error;

	async fn update(connection: impl 'async_trait + Executor<'_, Database = Self::Db>, entity: Self::Entity) -> Result<()>
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
