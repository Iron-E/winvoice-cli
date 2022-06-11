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
	use clinvoice_adapter::{
		schema::{ContactInfoAdapter, LocationAdapter, OrganizationAdapter},
		Deletable,
	};
	use clinvoice_match::{MatchContact, MatchSet};
	use clinvoice_schema::ContactKind;

	use crate::schema::{util, PgContactInfo, PgLocation, PgOrganization};

	// TODO: use fuzzing
	#[tokio::test]
	async fn delete()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let organization = PgOrganization::create(
			&connection,
			vec![
				(
					true,
					ContactKind::Phone("555-555-5555".into()),
					"Office Number".into(),
				),
				(
					true,
					ContactKind::Email("somethingsomething@invalid.com".into()),
					"Primary Email".into(),
				),
				(
					true,
					ContactKind::Email("foo@bar.io".into()),
					"Secondary Email".into(),
				),
			],
			earth.clone(),
			"Some Organization".into(),
		)
		.await
		.unwrap();

		PgContactInfo::delete(
			&connection,
			[&organization.contact_info[0], &organization.contact_info[1]].into_iter(),
		)
		.await
		.unwrap();

		assert_eq!(
			PgContactInfo::retrieve(
				&connection,
				&MatchSet::Contains(MatchContact {
					organization_id: organization.id.into(),
					..Default::default()
				})
			)
			.await
			.unwrap()[&organization.id]
				.as_slice(),
			&[organization.contact_info[2].clone()],
		);
	}
}
