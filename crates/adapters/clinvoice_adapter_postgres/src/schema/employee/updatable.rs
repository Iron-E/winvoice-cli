use clinvoice_adapter::Updatable;
use clinvoice_schema::Employee;
use sqlx::{Postgres, Result, Transaction};

use super::PgEmployee;

#[async_trait::async_trait]
impl Updatable for PgEmployee
{
	type Db = Postgres;
	type Entity = Employee;

	async fn update(connection: &mut Transaction<Self::Db>, entity: Self::Entity) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn update()
	{
		// TODO: write test
	}
}
