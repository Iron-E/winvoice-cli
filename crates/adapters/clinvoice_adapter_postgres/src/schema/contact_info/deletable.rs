use core::fmt::Display;

use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, TableToSql},
	schema::columns::ContactColumns,
	Deletable,
};
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
			s.push('(')
				.push_unseparated(ContactColumns::default().label)
				.push_unseparated('=')
				.push_bind(&c.label)
				.push_unseparated(')');
		}

		let mut peekable_entities = entities.peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let mut query = QueryBuilder::new(sql::DELETE);
		query
			.push(sql::FROM)
			.push(ContactColumns::<&str>::TABLE_NAME)
			.push(sql::WHERE);

		{
			let mut separated = query.separated(' ');

			if let Some(e) = peekable_entities.next()
			{
				write(&mut separated, e);
			}

			peekable_entities.for_each(|e| {
				separated.push_unseparated(sql::OR);
				write(&mut separated, e);
			});
		}

		query.prepare().execute(connection).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::{
		schema::{ContactInfoAdapter, LocationAdapter},
		Deletable,
	};
	use clinvoice_match::{MatchContact, MatchStr};
	use clinvoice_schema::{Contact, ContactKind};
	use pretty_assertions::assert_eq;

	use crate::schema::{util, PgContactInfo, PgLocation};

	#[tokio::test]
	async fn delete()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let contact_info = [
			Contact {
				label: "Office Number".into(),
				kind: ContactKind::Phone("555-555-5555".into()),
			},
			Contact {
				label: "Primary Email".into(),
				kind: ContactKind::Email("somethingsomething@invalid.com".into()),
			},
			Contact {
				label: "Mailing Address".into(),
				kind: ContactKind::Address(earth),
			},
		];

		PgContactInfo::create(&connection, contact_info.iter())
			.await
			.unwrap();
		PgContactInfo::delete(
			&connection,
			[&contact_info[0], &contact_info[1]].into_iter(),
		)
		.await
		.unwrap();

		assert_eq!(
			PgContactInfo::retrieve(&connection, &MatchContact {
				label: MatchStr::Or(
					contact_info
						.iter()
						.map(|c| c.label.clone().into())
						.collect()
				),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[contact_info[2].clone()],
		);

		// cleanup for the test; since labels are the primary key
		PgContactInfo::delete(&connection, [&contact_info[2]].into_iter())
			.await
			.unwrap();
	}
}
