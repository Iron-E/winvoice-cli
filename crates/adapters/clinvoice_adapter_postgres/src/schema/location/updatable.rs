use std::collections::LinkedList;

use clinvoice_adapter::{schema::columns::LocationColumns, Updatable};
use clinvoice_schema::Location;
use sqlx::{Postgres, Result, Transaction};

use super::PgLocation;
use crate::PgSchema;

#[async_trait::async_trait]
impl Updatable for PgLocation
{
	type Db = Postgres;
	type Entity = Location;

	async fn update<'e, 'i>(
		connection: &mut Transaction<Self::Db>,
		entities: impl 'async_trait + Clone + Iterator<Item = &'i Self::Entity> + Send,
	) -> Result<()>
	where
		'e: 'i,
		Self::Entity: 'e,
	{
		let mut entities_peekable = entities.peekable();

		// There is nothing to do.
		if entities_peekable.peek().is_none()
		{
			return Ok(());
		}

		let mut entities_by_outer: LinkedList<Vec<_>> = Default::default();
		entities_by_outer.push_back(entities_peekable.collect());

		loop
		{
			let mut outers = entities_by_outer
				.back()
				.unwrap()
				.iter()
				.filter_map(|e| e.outer.as_deref())
				.peekable();

			// There are no more outer locations, so we can stop looking for them in this loop.
			if outers.peek().is_none()
			{
				break;
			}

			let outers_collected = outers.collect();
			entities_by_outer.push_back(outers_collected);
		}

		let mut entities_collected: Vec<_> = entities_by_outer.into_iter().flatten().collect();

		// NOTE: we don't want to update a given row in the DB more than once.
		// PERF: we can only get duplicates if there is more than one entitiy to update.
		if entities_collected.len() > 1
		{
			// PERF: `dedup` needs a list to be sorted. there's no way for two duplicates to get
			//       unsorted unless there are at least three elements.
			if entities_collected.len() > 2
			{
				entities_collected.sort();
			}

			entities_collected.dedup();
		}

		const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();
		PgSchema::update(&mut *connection, COLUMNS, "locations", "L", "V", |query| {
			query.push_values(entities_collected.iter(), |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(e.outer.as_ref().map(|o| o.id));
			});
		})
		.await?;

		// TODO: make this function recursive once `async` traits are stable.
		//       at that point, it will be possible to remove `entities_collected` and instead use
		//       `entities.clone().peekable()` in its place.
		// Self::update(connection, entities.filter_map(|e| e.outer.as_deref())).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::{schema::LocationAdapter, Updatable};
	use clinvoice_match::MatchLocation;

	use crate::schema::{util, PgLocation};

	// TODO: use fuzzing
	#[tokio::test]
	async fn update()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let mut usa = PgLocation::create(&connection, "USA".into(), Some(earth.clone()))
			.await
			.unwrap();

		let (mut arizona, mut utah) = futures::try_join!(
			PgLocation::create(&connection, "Arizona".into(), Some(usa.clone())),
			PgLocation::create(&connection, "Utah".into(), Some(usa.clone())),
		)
		.unwrap();

		usa.name = "Not USA".into();
		arizona.name = "Not Arizona".into();

		arizona.outer = Some(usa.clone().into());
		utah.outer = Some(usa.into());

		{
			let mut transaction = connection.begin().await.unwrap();
			PgLocation::update(&mut transaction, [arizona.clone(), utah.clone()].iter())
				.await
				.unwrap();
			transaction.commit().await.unwrap();
		}

		let arizona_db = PgLocation::retrieve(&connection, &MatchLocation {
			id: arizona.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.remove(0);

		let utah_db = PgLocation::retrieve(&connection, &MatchLocation {
			id: utah.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.remove(0);

		assert_eq!(arizona, arizona_db);
		assert_eq!(utah, utah_db);
	}
}
