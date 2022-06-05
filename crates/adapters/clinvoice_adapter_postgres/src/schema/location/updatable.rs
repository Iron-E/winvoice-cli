use clinvoice_adapter::{schema::columns::LocationColumns, Updatable};
use clinvoice_schema::Location;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgLocation;

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
		const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();
		const TABLE_IDENT: &str = "L";
		const VALUES_IDENT: &str = "V";

		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let values_columns = COLUMNS.scoped(VALUES_IDENT);

		let mut query = QueryBuilder::new("UPDATE locations AS ");

		query
			.separated(' ')
			.push(TABLE_IDENT)
			.push("SET")
			.push(COLUMNS.name)
			.push_unseparated('=')
			.push_unseparated(values_columns.name)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.outer_id)
			.push_unseparated('=')
			.push_unseparated(values_columns.outer_id)
			.push("FROM (");

		query.push_values(peekable_entities, |mut q, e| {
			q.push_bind(e.id)
				.push_bind(&e.name)
				.push_bind(e.outer.as_ref().map(|o| o.id));
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push_unseparated(COLUMNS.id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.name)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.outer_id)
			.push_unseparated(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push_unseparated('=')
			.push_unseparated(values_columns.id);

		query.push(';').build().execute(&mut *connection).await?;

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
