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

		PgSchema::update(connection, OrganizationColumns::default(), |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(e.location.id)
					.push_bind(&e.name);
			});
		})
		.await?;

		PgLocation::update(connection, entities.map(|e| &e.location)).await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::{
		schema::{LocationAdapter, OrganizationAdapter},
		Updatable,
	};
	use clinvoice_match::MatchOrganization;
	use pretty_assertions::assert_eq;

	use crate::schema::{util, PgLocation, PgOrganization};

	#[tokio::test]
	async fn update()
	{
		let connection = util::connect().await;

		let (earth, mars) = futures::try_join!(
			PgLocation::create(&connection, "Earth".into(), None),
			PgLocation::create(&connection, "Mars".into(), None),
		)
		.unwrap();

		let mut organization = PgOrganization::create(&connection, earth, "Some Organization".into())
			.await
			.unwrap();

		organization.location = mars;
		organization.name = format!("Not {}", organization.name);

		{
			let mut transaction = connection.begin().await.unwrap();
			PgOrganization::update(&mut transaction, [&organization].into_iter())
				.await
				.unwrap();
			transaction.commit().await.unwrap();
		}

		assert_eq!(
			PgOrganization::retrieve(&connection, &MatchOrganization {
				id: organization.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[organization]
		);
	}
}
