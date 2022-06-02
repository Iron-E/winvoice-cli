use core::fmt::Display;

use clinvoice_adapter::Deletable;
use clinvoice_schema::Contact;
use sqlx::{query_builder::Separated, Executor, Postgres, QueryBuilder, Result};

use super::PgContactInfo;
use crate::schema::contact_info::columns::PgContactColumns;

#[async_trait::async_trait]
impl Deletable for PgContactInfo
{
	type Db = Postgres;
	type Entity = Contact;

	async fn delete(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = Self::Entity> + Send,
	) -> Result<()>
	{
		let mut query = QueryBuilder::new("DELETE FROM contact_information WHERE ");

		{
			let mut separated = query.separated(' ');

			fn write<T>(s: &mut Separated<Postgres, T>, c: Contact)
			where
				T: Display,
			{
				const COLUMNS: PgContactColumns<&'static str> = PgContactColumns::new();

				s.push('(')
					.push(COLUMNS.organization_id)
					.push('=')
					.push(c.organization_id)
					.push("AND")
					.push(COLUMNS.label)
					.push('=')
					.push_bind(c.label)
					.push(')');
			}

			let mut entities_mut = entities;
			if let Some(e) = entities_mut.next()
			{
				write(&mut separated, e);
			}

			entities_mut.for_each(|e| {
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
