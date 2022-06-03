use clinvoice_adapter::Updatable;
use clinvoice_schema::Location;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgLocation;
use crate::schema::{location::columns::PgLocationColumns, PgOption};

#[async_trait::async_trait]
impl Updatable for PgLocation
{
	type Db = Postgres;
	type Entity = Location;

	async fn update<'e>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'e Self::Entity> + Send,
	) -> Result<()>
	where
		Self::Entity: 'e,
	{
		const COLUMNS: PgLocationColumns<&'static str> = PgLocationColumns::new();
		const TABLE_IDENT: &'static str = "L";
		const VALUES_IDENT: &'static str = "V";

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
			.push('=')
			.push(values_columns.name)
			.push(',')
			.push(COLUMNS.outer_id)
			.push('=')
			.push(values_columns.outer_id)
			.push("FROM (");

		query.push_values(peekable_entities, |mut q, e| {
			q.push(e.id)
				.push_bind(&e.name)
				.push(PgOption(e.outer.as_ref().map(|o| o.id)));
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push(COLUMNS.id)
			.push(',')
			.push(COLUMNS.name)
			.push(',')
			.push(COLUMNS.outer_id)
			.push(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push('=')
			.push(values_columns.id);

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
