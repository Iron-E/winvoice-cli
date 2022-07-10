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

		PgSchema::update(connection, ContactColumns::default(), |query| {
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
	use std::collections::HashSet;

	use clinvoice_adapter::{
		schema::{ContactInfoAdapter, LocationAdapter},
		Deletable,
		Updatable,
	};
	use clinvoice_match::{MatchContact, MatchStr};
	use clinvoice_schema::{Contact, ContactKind};
	use pretty_assertions::assert_eq;

	use crate::schema::{util, PgContactInfo, PgLocation};

	#[tokio::test]
	async fn update()
	{
		let connection = util::connect().await;

		let (earth, mars) = futures::try_join!(
			PgLocation::create(&connection, "Earth".into(), None),
			PgLocation::create(&connection, "Mars".into(), None),
		)
		.unwrap();

		let mut contact_info = [
			Contact {
				kind: ContactKind::Address(earth),
				label: "asldkjalskfhalskdj Office".into(),
			},
			Contact {
				kind: ContactKind::Phone("1-800-555-5555".into()),
				label: "gbtyufs buai Primary Contact".into(),
			},
		];

		PgContactInfo::create(&connection, contact_info.iter())
			.await
			.unwrap();

		contact_info[0].kind = ContactKind::Address(mars);
		contact_info[1].kind = ContactKind::Email("foo@bar.io".into());

		{
			let mut transaction = connection.begin().await.unwrap();
			PgContactInfo::update(&mut transaction, contact_info.iter())
				.await
				.unwrap();
			transaction.commit().await.unwrap();
		}

		let db_contact_info: HashSet<_> = PgContactInfo::retrieve(&connection, &MatchContact {
			label: MatchStr::Or(
				contact_info
					.iter()
					.map(|c| c.label.clone().into())
					.collect(),
			),
			..Default::default()
		})
		.await
		.unwrap()
		.into_iter()
		.collect();

		// cleanup
		PgContactInfo::delete(&connection, contact_info.iter())
			.await
			.unwrap();

		assert_eq!(
			contact_info.into_iter().collect::<HashSet<_>>(),
			db_contact_info
		);
	}
}
