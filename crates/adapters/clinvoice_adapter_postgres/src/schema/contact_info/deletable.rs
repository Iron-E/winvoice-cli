use clinvoice_adapter::Deletable;
use clinvoice_schema::Contact;
use sqlx::{Executor, Postgres, Result};

use super::PgContactInfo;

#[async_trait::async_trait]
impl Deletable for PgContactInfo
{
	type Db = Postgres;
	type Entity = Contact;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn delete()
	{
		// TODO: write test
	}
}
