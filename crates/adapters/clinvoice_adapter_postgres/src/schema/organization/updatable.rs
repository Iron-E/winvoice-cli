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
		PgSchema::update(connection, COLUMNS, "organizations", "O", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(e.location.id)
					.push_bind(&e.name);
			});
		})
		.await?;

		PgLocation::update(connection, entities.map(|e| &e.location)).await?;

		Ok(())
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

		let (mut organization, mut organization2) = futures::try_join!(
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into(),),
			PgOrganization::create(&connection, earth.clone(), "Some Other Organization".into(),),
		)
		.unwrap();

		organization.location.name = format!("Not {}", earth.name);
		organization.name = format!("Not {}", organization.name);

		organization2.location = mars;

		{
			let mut transaction = connection.begin().await.unwrap();
			PgOrganization::update(
				&mut transaction,
				[&organization, &organization2].into_iter(),
			)
			.await
			.unwrap();
			transaction.commit().await.unwrap();
		}

		let organization_db = PgOrganization::retrieve(&connection, &MatchOrganization {
			id: organization.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.remove(0);

		let organization2_db = PgOrganization::retrieve(&connection, &MatchOrganization {
			id: organization2.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.remove(0);

		assert_eq!(organization, organization_db);
		assert_eq!(organization2, organization2_db);
	}
}
