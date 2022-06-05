use clinvoice_adapter::{schema::columns::OrganizationColumns, Updatable};
use clinvoice_schema::Organization;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgOrganization;
use crate::schema::PgLocation;

#[async_trait::async_trait]
impl Updatable for PgOrganization
{
	type Db = Postgres;
	type Entity = Organization;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		const COLUMNS: OrganizationColumns<&'static str> = OrganizationColumns::default();
		const TABLE_IDENT: &str = "O";
		const VALUES_IDENT: &str = "V";

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
			.push_unseparated('=')
			.push_unseparated(values_columns.location_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.name)
			.push_unseparated('=')
			.push_unseparated(values_columns.name)
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
			.push_unseparated(COLUMNS.id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.location_id)
			.push_unseparated(',')
			.push_unseparated(COLUMNS.name)
			.push_unseparated(')')
			.push("WHERE")
			.push(COLUMNS.scoped(TABLE_IDENT).id)
			.push_unseparated('=')
			.push_unseparated(values_columns.id);

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
