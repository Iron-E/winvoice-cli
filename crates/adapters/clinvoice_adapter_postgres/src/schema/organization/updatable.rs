use clinvoice_adapter::{schema::columns::OrganizationColumns, Updatable};
use clinvoice_schema::Organization;
use sqlx::{Postgres, Result, Transaction};

use super::PgOrganization;
use crate::{schema::PgLocation, PgSchema};

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
		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: OrganizationColumns<&'static str> = OrganizationColumns::default();
		PgSchema::update(
			&mut *connection,
			COLUMNS,
			"organizations",
			"O",
			"V",
			|query| {
				query.push_values(peekable_entities, |mut q, e| {
					q.push_bind(e.id)
						.push_bind(e.location.id)
						.push_bind(&e.name);
				});
			},
		)
		.await?;

		PgLocation::update(connection, entities.map(|e| &e.location)).await
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn update()
	{
		// TODO: write test
	}
}
