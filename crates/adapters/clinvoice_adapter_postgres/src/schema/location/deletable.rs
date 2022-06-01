use clinvoice_adapter::{Deletable, WriteWhereClause};
use clinvoice_match::Match;
use clinvoice_schema::Location;
use sqlx::{Executor, Postgres, QueryBuilder, Result};

use super::PgLocation;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgLocation
{
	type Db = Postgres;
	type Entity = Location;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		let mut query = QueryBuilder::new("DELETE FROM locations");

		PgSchema::write_where_clause(
			Default::default(),
			"id",
			&Match::Or(entities.map(|e| e.id.into()).collect()),
			&mut query,
		);

		query.push(';').build().execute(connection).await?;

		Ok(())
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
