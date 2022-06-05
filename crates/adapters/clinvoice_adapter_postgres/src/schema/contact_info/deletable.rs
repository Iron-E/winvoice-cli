use core::fmt::Display;

use clinvoice_adapter::{schema::columns::ContactColumns, Deletable};
use clinvoice_schema::Contact;
use sqlx::{query_builder::Separated, Executor, Postgres, QueryBuilder, Result};

use super::PgContactInfo;

#[async_trait::async_trait]
impl Deletable for PgContactInfo
{
	type Db = Postgres;
	type Entity = Contact;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		fn write<'query, 'args, T>(s: &mut Separated<'query, 'args, Postgres, T>, c: &'args Contact)
		where
			T: Display,
		{
			const COLUMNS: ContactColumns<&'static str> = ContactColumns::default();

			s.push('(')
				.push(COLUMNS.organization_id)
				.push_unseparated('=')
				.push_unseparated(c.organization_id)
				.push("AND")
				.push(COLUMNS.label)
				.push_unseparated('=')
				.push_bind(&c.label)
				.push(')');
		}

		let mut peekable_entities = entities.peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let mut query = QueryBuilder::new("DELETE FROM contact_information WHERE ");

		{
			let mut separated = query.separated(' ');

			if let Some(e) = peekable_entities.next()
			{
				write(&mut separated, e);
			}

			peekable_entities.for_each(|e| {
				separated.push("OR");
				write(&mut separated, e);
			});
		}

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
