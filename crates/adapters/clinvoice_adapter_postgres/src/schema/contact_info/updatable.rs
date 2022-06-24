use clinvoice_adapter::{schema::columns::ContactColumns, Updatable};
use clinvoice_schema::Contact;
use sqlx::{Postgres, Result, Transaction};

use super::PgContactInfo;
use crate::PgSchema;

#[async_trait::async_trait]
impl Updatable for PgContactInfo
{
	type Db = Postgres;
	type Entity = Contact;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		let mut peekable_entities = entities.peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: ContactColumns<&'static str> = ContactColumns::default();
		PgSchema::update(connection, COLUMNS, "contact_information", "C", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.kind.address().map(|a| a.id))
					.push_bind(e.kind.email())
					.push_bind(&e.label)
					.push_bind(e.kind.other())
					.push_bind(e.kind.phone());
			});
		})
		.await
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn update()
	{
		todo!("write test")
	}
}
