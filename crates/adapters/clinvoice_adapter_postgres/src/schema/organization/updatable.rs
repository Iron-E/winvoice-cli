use clinvoice_adapter::Updatable;
use clinvoice_schema::Organization;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgOrganization;
use crate::schema::{organization::columns::PgOrganizationColumns, PgLocation};

#[async_trait::async_trait]
impl Updatable for PgOrganization
{
	type Db = Postgres;
	type Entity = Organization;

	async fn update<'e>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'e Self::Entity> + Send,
	) -> Result<()>
	where
		Self::Entity: 'e,
	{
		const COLUMNS: PgOrganizationColumns<&'static str> = PgOrganizationColumns::new();
		const TABLE_IDENT: &'static str = "O";
		const VALUES_IDENT: &'static str = "V";

		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let values_columns = COLUMNS.scoped(VALUES_IDENT);

		let mut query = QueryBuilder::new("UPDATE organizations AS ");

		query
			.separated(' ')
			.push(TABLE_IDENT)
			.push("SET")
			.push(COLUMNS.location_id)
			.push('=')
			.push(values_columns.location_id)
			.push(',')
			.push(COLUMNS.name)
			.push('=')
			.push(values_columns.name)
			.push("FROM (");

		query.push_values(peekable_entities, |mut q, e| {
			q.push_bind(e.id)
				.push_bind(e.location.id)
				.push_bind(&e.name);
		});

		query
			.separated(' ')
			.push(") AS")
			.push(VALUES_IDENT)
			.push('(')
			.push(COLUMNS.id)
			.push(',')
			.push(COLUMNS.location_id)
			.push(',')
			.push(COLUMNS.name)
			.push(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push('=')
			.push(values_columns.id);

		query.push(';').build().execute(&mut *connection).await?;

		PgLocation::update(connection, entities.map(|e| &e.location)).await?;

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
