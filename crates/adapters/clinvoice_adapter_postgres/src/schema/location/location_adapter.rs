use clinvoice_adapter::{
	schema::LocationAdapter,
	WriteContext,
	WriteFromClause,
	WriteSelectClause,
	WriteWhereClause,
};
use clinvoice_match::MatchLocation;
use clinvoice_schema::{views::LocationView, Location};
use futures::TryStreamExt;
use sqlx::{PgPool, Result, Row};

use super::PostgresLocation;
use crate::PostgresSchema as Schema;

#[async_trait::async_trait]
impl LocationAdapter for PostgresLocation
{
	async fn create(connection: &PgPool, name: String) -> Result<Location>
	{
		let row = sqlx::query!(
			"INSERT INTO locations (name) VALUES ($1) RETURNING id;",
			name
		)
		.fetch_one(connection)
		.await?;

		Ok(Location {
			id: row.id,
			name,
			outer_id: None,
		})
	}

	async fn create_inner(connection: &PgPool, outer: &Location, name: String) -> Result<Location>
	{
		let row = sqlx::query!(
			"INSERT INTO locations (name, outer_id) VALUES ($1, $2) RETURNING id;",
			name,
			outer.id
		)
		.fetch_one(connection)
		.await?;

		Ok(Location {
			id: row.id,
			name,
			outer_id: Some(outer.id),
		})
	}

	// WARN: `Might need `Acquire` or `&mut Transaction` depending on how recursive views work
	async fn retrieve_view(
		connection: &PgPool,
		match_condition: &MatchLocation,
	) -> Result<Vec<LocationView>>
	{
		let mut query = Schema::write_select_clause(["L.name", "L.outer_id", "L.id"]);
		Schema::write_from_clause(&mut query, "locations", "L");
		Schema::write_where_clause(
			WriteContext::BeforeWhereClause,
			"L",
			match_condition,
			&mut query,
		);
		query.push(';');

		sqlx::query(&query)
			.fetch(connection)
			.and_then(|row| PostgresLocation::retrieve_view_by_id(connection, row.get("id")))
			.try_collect()
			.await
	}
}

#[cfg(test)]
mod tests
{
	use std::{borrow::Cow::Owned, collections::HashSet};

	use clinvoice_match::{Match, MatchLocation, MatchStr, MatchOuterLocation};
	use clinvoice_schema::views::LocationView;

	use super::{LocationAdapter, PostgresLocation};
	use crate::schema::util;

	/// TODO: use fuzzing
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn create()
	{
		let connection = util::connect().await;

		let earth = PostgresLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PostgresLocation::create_inner(&connection, &earth, "USA".into())
			.await
			.unwrap();

		let arizona = PostgresLocation::create_inner(&connection, &usa, "Arizona".into())
			.await
			.unwrap();

		// Assert ::create_inner works when `outer_id` has already been used for another `Location`
		assert!(
			PostgresLocation::create_inner(&connection, &usa, "Utah".into())
				.await
				.is_ok()
		);

		macro_rules! select {
			($id:expr) => {
				sqlx::query!("SELECT * FROM locations WHERE id = $1", $id)
					.fetch_one(&connection)
					.await
					.unwrap()
			};
		}

		let database_earth = select!(earth.id);

		// Assert ::create writes accurately to the DB
		assert_eq!(earth.id, database_earth.id);
		assert_eq!(earth.name, database_earth.name);
		assert_eq!(earth.outer_id, None);
		assert_eq!(earth.outer_id, database_earth.outer_id);

		let database_usa = select!(usa.id);

		// Assert ::create_inner writes accurately to the DB when `outer_id` is `None`
		assert_eq!(usa.id, database_usa.id);
		assert_eq!(usa.name, database_usa.name);
		assert_eq!(usa.outer_id, Some(earth.id));
		assert_eq!(usa.outer_id, database_usa.outer_id);

		let database_arizona = select!(arizona.id);

		// Assert ::create_inner writes accurately to the DB when `outer_id` is `Some(…)`
		assert_eq!(arizona.id, database_arizona.id);
		assert_eq!(arizona.name, database_arizona.name);
		assert_eq!(arizona.outer_id, Some(usa.id));
		assert_eq!(arizona.outer_id, database_arizona.outer_id);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn retrieve_view()
	{
		let connection = util::connect().await;

		let earth = PostgresLocation::create(&connection, "Earth".into())
			.await
			.unwrap();

		let usa = PostgresLocation::create_inner(&connection, &earth, "USA".into())
			.await
			.unwrap();

		let arizona = PostgresLocation::create_inner(&connection, &usa, "Arizona".into())
			.await
			.unwrap();

		let utah = PostgresLocation::create_inner(&connection, &usa, "Utah".into())
			.await
			.unwrap();

		let earth_view = LocationView {
			id: earth.id,
			name: earth.name.clone(),
			outer: None,
		};

		let usa_view = LocationView {
			id: usa.id,
			name: usa.name.clone(),
			outer: Some(earth_view.clone().into()),
		};

		let arizona_view = LocationView {
			id: arizona.id,
			name: arizona.name.clone(),
			outer: Some(usa_view.clone().into()),
		};

		let utah_view = LocationView {
			id: utah.id,
			name: utah.name.clone(),
			outer: Some(usa_view.clone().into()),
		};

		// Assert ::retrieve_view retrieves accurately from the DB
		assert_eq!(
			&[earth_view],
			PostgresLocation::retrieve_view(&connection, &MatchLocation {
				id: earth.id.into(),
				..Default::default()
			})
			.await
			.unwrap()
			.as_slice()
		);

		assert_eq!(
			[utah_view, arizona_view].into_iter().collect::<HashSet<_>>(),
			PostgresLocation::retrieve_view(&connection, &MatchLocation {
				outer: MatchOuterLocation::Some(Box::new(MatchLocation {
					id: usa_view.id.into(),
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
