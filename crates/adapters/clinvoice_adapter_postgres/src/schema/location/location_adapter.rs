use clinvoice_adapter::{
	fmt::{sql, QueryBuilderExt, TableToSql},
	schema::{columns::LocationColumns, LocationAdapter},
};
use clinvoice_match::MatchLocation;
use clinvoice_schema::Location;
use futures::TryStreamExt;
use sqlx::{PgPool, Result, Row};

use super::PgLocation;
use crate::fmt::PgLocationRecursiveCte;

#[async_trait::async_trait]
impl LocationAdapter for PgLocation
{
	async fn create(connection: &PgPool, name: String, outer: Option<Location>) -> Result<Location>
	{
		let row = sqlx::query!(
			"INSERT INTO locations (name, outer_id) VALUES ($1, $2) RETURNING id;",
			name,
			outer.as_ref().map(|o| o.id)
		)
		.fetch_one(connection)
		.await?;

		Ok(Location {
			id: row.id,
			name,
			outer: outer.map(|o| o.into()),
		})
	}

	async fn retrieve(connection: &PgPool, match_condition: &MatchLocation)
		-> Result<Vec<Location>>
	{
		const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();

		let mut query = Self::query_with_recursive(match_condition);

		query
			.push(sql::SELECT)
			.push(COLUMNS.default_scope().id)
			.push_from(
				PgLocationRecursiveCte::from(match_condition),
				LocationColumns::<char>::DEFAULT_ALIAS,
			)
			.prepare()
			.fetch(connection)
			.and_then(|row| PgLocation::retrieve_by_id(connection, row.get(COLUMNS.id)))
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::collections::HashSet;

	use clinvoice_match::{MatchLocation, MatchOuterLocation};
	use pretty_assertions::assert_eq;

	use super::{LocationAdapter, PgLocation};
	use crate::schema::util;

	#[tokio::test]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let usa = PgLocation::create(&connection, "USA".into(), Some(earth.clone()))
			.await
			.unwrap();

		let (arizona, utah) = futures::try_join!(
			PgLocation::create(&connection, "Arizona".into(), Some(usa.clone())),
			PgLocation::create(&connection, "Utah".into(), Some(usa.clone())),
		)
		.unwrap();

		macro_rules! select {
			($id:expr) => {
				sqlx::query!("SELECT * FROM locations WHERE id = $1", $id)
					.fetch_one(&connection)
					.await
					.unwrap()
			};
		}

		// Assert ::create writes accurately to the DB
		let database_earth = select!(earth.id);
		assert_eq!(earth.id, database_earth.id);
		assert_eq!(earth.name, database_earth.name);
		assert_eq!(earth.outer, None);
		assert_eq!(earth.outer.map(|o| o.id), database_earth.outer_id);

		// Assert ::create_inner writes accurately to the DB when `outer_id` is `None`
		let database_usa = select!(usa.id);
		assert_eq!(usa.id, database_usa.id);
		assert_eq!(usa.name, database_usa.name);
		let usa_outer_id = usa.outer.map(|o| o.id);
		assert_eq!(usa_outer_id, Some(earth.id));
		assert_eq!(usa_outer_id, database_usa.outer_id);

		// Assert ::create_inner writes accurately to the DB when `outer_id` is `Some(…)`
		let database_arizona = select!(arizona.id);
		assert_eq!(arizona.id, database_arizona.id);
		assert_eq!(arizona.name, database_arizona.name);
		let arizona_outer_id = arizona.outer.map(|o| o.id);
		assert_eq!(arizona_outer_id, Some(usa.id));
		assert_eq!(arizona_outer_id, database_arizona.outer_id);

		let database_utah = select!(utah.id);
		assert_eq!(utah.id, database_utah.id);
		assert_eq!(utah.name, database_utah.name);
		let utah_outer_id = utah.outer.map(|o| o.id);
		assert_eq!(utah_outer_id, Some(usa.id));
		assert_eq!(utah_outer_id, database_utah.outer_id);
	}

	#[tokio::test]
	async fn retrieve()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let usa = PgLocation::create(&connection, "USA".into(), Some(earth.clone()))
			.await
			.unwrap();

		let (arizona, utah) = futures::try_join!(
			PgLocation::create(&connection, "Arizona".into(), Some(usa.clone())),
			PgLocation::create(&connection, "Utah".into(), Some(usa.clone())),
		)
		.unwrap();

		// Assert ::retrieve retrieves accurately from the DB
		assert_eq!(
			PgLocation::retrieve(&connection, &MatchLocation {
				id: earth.id.into(),
				outer: MatchOuterLocation::None,
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice(),
			&[earth]
		);

		assert_eq!(
			[utah, arizona].into_iter().collect::<HashSet<_>>(),
			PgLocation::retrieve(&connection, &MatchLocation {
				outer: MatchOuterLocation::Some(Box::new(MatchLocation {
					id: usa.id.into(),
					..Default::default()
				})),
				..Default::default()
			})
			.await
			.unwrap()
			.into_iter()
			.collect::<HashSet<_>>()
		);
	}
}
