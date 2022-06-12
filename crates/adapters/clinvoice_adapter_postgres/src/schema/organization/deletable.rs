use clinvoice_adapter::Deletable;
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
		// TODO: use `for<'a> |o: &'a Organization| o.id`
		fn mapper(o: &Organization) -> Id
		{
			o.id
		}

		PgSchema::delete(connection, "organizations", entities.map(mapper)).await
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashMap;

	use clinvoice_adapter::{
		schema::{ContactInfoAdapter, LocationAdapter, OrganizationAdapter},
		Deletable,
	};
	use clinvoice_match::{Match, MatchContact, MatchOrganization, MatchSet};
	use clinvoice_schema::ContactKind;

	use crate::schema::{util, PgContactInfo, PgLocation, PgOrganization};

	#[tokio::test]
	async fn delete()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let (organization, organization2, organization3) = futures::try_join!(
			PgOrganization::create(
				&connection,
				vec![(
					true,
					ContactKind::Phone("555-555-5555".into()),
					"Office Number".into()
				)],
				earth.clone(),
				"Some Organization".into(),
			),
			PgOrganization::create(
				&connection,
				Vec::new(),
				earth.clone(),
				"Some Other Organization".into(),
			),
			PgOrganization::create(
				&connection,
				Vec::new(),
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

		assert_eq!(
			PgContactInfo::retrieve(
				&connection,
				&MatchSet::Contains(MatchContact {
					organization_id: Match::Or(vec![organization.id.into()]),
					..Default::default()
				})
			)
			.await
			.unwrap(),
			HashMap::new(),
		);
	}
}
