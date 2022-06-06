use clinvoice_adapter::{schema::columns::LocationColumns, Updatable};
use clinvoice_schema::Location;
use sqlx::{Postgres, Result, Transaction};

use super::PgLocation;
use crate::PgSchema;

#[async_trait::async_trait]
impl Updatable for PgLocation
{
	type Db = Postgres;
	type Entity = Location;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();
		PgSchema::update(&mut *connection, COLUMNS, "locations", "L", "V", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(e.outer.as_ref().map(|o| o.id));
			});
		})
		.await?;

		Self::update(connection, entities.filter_map(|e| e.outer.as_deref())).await?;

		Ok(())
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
