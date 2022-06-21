use clinvoice_adapter::{
	fmt::{ColumnsToSql, SnakeCase},
	schema::{
		columns::{LocationColumns, OrganizationColumns},
		OrganizationAdapter,
	},
	WriteWhereClause,
};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::{Location, Organization};
use futures::TryStreamExt;
use sqlx::{PgPool, Result};

use super::PgOrganization;
use crate::{fmt::PgLocationRecursiveCte, schema::PgLocation, PgSchema};

#[async_trait::async_trait]
impl OrganizationAdapter for PgOrganization
{
	async fn create(connection: &PgPool, location: Location, name: String) -> Result<Organization>
	{
		let row = sqlx::query!(
			"INSERT INTO organizations (location_id, name) VALUES ($1, $2) RETURNING id;",
			location.id,
			name
		)
		.fetch_one(connection)
		.await?;

		Ok(Organization {
			id: row.id,
			location,
			name,
		})
	}

	async fn retrieve(
		connection: &PgPool,
		match_condition: &MatchOrganization,
	) -> Result<Vec<Organization>>
	{
		const ALIAS: &str = "O";
		const COLUMNS: OrganizationColumns<&'static str> = OrganizationColumns::default();
		const LOCATION_ALIAS: SnakeCase<&str, &str> = SnakeCase::Body(ALIAS, "L");
		const LOCATION_COLUMNS: LocationColumns<&'static str> = LocationColumns::default();

		let columns = COLUMNS.scoped(ALIAS);
		let location_columns = LOCATION_COLUMNS.scoped(LOCATION_ALIAS);
		let mut query = PgLocation::query_with_recursive(&match_condition.location);

		query.push("SELECT ");
		columns.push(&mut query);
		query
			.separated(' ')
			.push(" FROM organizations")
			.push(ALIAS)
			.push("JOIN")
			.push(PgLocationRecursiveCte::from(&match_condition.location))
			.push(LOCATION_ALIAS)
			.push("ON (");

		query
			.separated('=')
			.push(location_columns.id)
			.push(columns.location_id)
			.push_unseparated(')');

		PgSchema::write_where_clause(Default::default(), "O", match_condition, &mut query);

		query
			.push(';')
			.build()
			.fetch(connection)
			.and_then(
				|row| async move { PgOrganization::row_to_view(connection, COLUMNS, &row).await },
			)
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashSet;

	use clinvoice_adapter::schema::LocationAdapter;
	use clinvoice_match::{Match, MatchLocation, MatchOrganization, MatchOuterLocation};

	use super::{OrganizationAdapter, PgOrganization};
	use crate::schema::{util, PgLocation};

	#[tokio::test]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let organization =
			PgOrganization::create(&connection, earth.clone(), "Some Organization".into())
				.await
				.unwrap();

		let row = sqlx::query!(
			"SELECT * FROM organizations WHERE id = $1;",
			organization.id
		)
		.fetch_one(&connection)
		.await
		.unwrap();

		// Assert ::create writes accurately to the DB
		assert_eq!(organization.id, row.id);
		assert_eq!(organization.location.id, earth.id);
		assert_eq!(organization.location.id, row.location_id);
		assert_eq!(organization.name, row.name);
	}

	#[tokio::test]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let usa = PgLocation::create(&connection, "USA".into(), Some(earth))
			.await
			.unwrap();

		let (arizona, utah) = futures::try_join!(
			PgLocation::create(&connection, "Arizona".into(), Some(usa.clone())),
			PgLocation::create(&connection, "Utah".into(), Some(usa.clone())),
		)
		.unwrap();

		let (organization, organization2) = futures::try_join!(
			PgOrganization::create(&connection, arizona.clone(), "Some Organization".into(),),
			PgOrganization::create(&connection, utah, "Some Other Organizatión".into(),),
		)
		.unwrap();

		// Assert ::retrieve gets the right data from the DB
		assert_eq!(
			PgOrganization::retrieve(&connection, &MatchOrganization {
				id: organization.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[organization.clone()],
		);

		assert_eq!(
			PgOrganization::retrieve(&connection, &MatchOrganization {
				location: MatchLocation {
					outer: MatchOuterLocation::Some(
						MatchLocation {
							id: Match::InRange(usa.id - 1, usa.id + 1),
							name: usa.name.into(),
							..Default::default()
						}
						.into()
					),
					..Default::default()
				},
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>(),
			[organization, organization2].into_iter().collect(),
		);
	}
}
