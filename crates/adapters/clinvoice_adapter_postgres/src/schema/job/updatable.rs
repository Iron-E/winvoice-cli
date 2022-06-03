use clinvoice_adapter::Updatable;
use clinvoice_schema::Job;
use sqlx::{Postgres, Result, Transaction};

use super::PgJob;

#[async_trait::async_trait]
impl Updatable for PgJob
{
	type Db = Postgres;
	type Entity = Job;

	async fn update<'e>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'e Self::Entity> + Send,
	) -> Result<()>
	where
		Self::Entity: 'e,
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
