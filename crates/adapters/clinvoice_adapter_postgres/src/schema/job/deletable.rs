use clinvoice_adapter::Deletable;
use clinvoice_schema::{Id, Job};
use sqlx::{Executor, Postgres, Result};

use super::PgJob;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgJob
{
	type Db = Postgres;
	type Entity = Job;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		// TODO: use `for<'a> |j: &'a Job| j.id`
		fn mapper(j: &Job) -> Id
		{
			j.id
		}

		PgSchema::delete(connection, "jobs", entities.map(mapper)).await
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
