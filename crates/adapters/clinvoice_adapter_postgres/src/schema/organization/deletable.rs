use clinvoice_adapter::{schema::columns::OrganizationColumns, Deletable};
use clinvoice_schema::{Id, Organization};
use sqlx::{Executor, Postgres, Result};

use super::PgOrganization;
use crate::PgSchema;

#[async_trait::async_trait]
impl Deletable for PgOrganization
{
	type Db = Postgres;
	type Entity = Organization;

	async fn delete<'e, 'i>(
		connection: impl 'async_trait + Executor<'_, Database = Self::Db>,
		entities: impl 'async_trait + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		fn mapper(o: &Organization) -> Id
		{
			o.id
		}

		// TODO: use `for<'a> |e: &'a Organization| e.id`
		PgSchema::delete::<_, _, OrganizationColumns<char>>(connection, entities.map(mapper)).await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::{
		schema::{LocationAdapter, OrganizationAdapter},
		Deletable,
	};
	use clinvoice_match::{Match, MatchOrganization};
	use pretty_assertions::assert_eq;

	use crate::schema::{util, PgLocation, PgOrganization};

	#[tokio::test]
	async fn delete()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let (organization, organization2, organization3) = futures::try_join!(
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into()),
			PgOrganization::create(&connection, earth.clone(), "Some Other Organization".into()),
			PgOrganization::create(
				&connection,
				earth.clone(),
				"Another Other Organization".into(),
			),
		)
		.unwrap();

		// The `organization`s still depend on `earth`
		assert!(PgLocation::delete(&connection, [&earth].into_iter())
			.await
			.is_err());
		PgOrganization::delete(&connection, [&organization, &organization2].into_iter())
			.await
			.unwrap();

		assert_eq!(
			PgOrganization::retrieve(&connection, &MatchOrganization {
				id: Match::Or(vec![
					organization.id.into(),
					organization2.id.into(),
					organization3.id.into()
				]),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[organization3]
		);
	}
}
