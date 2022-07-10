use std::{cmp::Ordering, collections::LinkedList};

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

		let mut entities_by_outer = LinkedList::<Vec<_>>::default();
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
				entities_collected.sort_by(|lhs, rhs| match rhs.id.cmp(&lhs.id)
				{
					// NOTE: this allows `dedup_by_key` prune edits to `Location`s which would overwrite
					//       the `Location`s which were initially passed to the function (e.g. if Earth
					//       and Sweden are both passed in to this function, Earth will take precedence
					//       over Sweden's copy of Earth).
					Ordering::Equal => Ordering::Greater,
					o => o,
				});
			}

			entities_collected.dedup_by_key(|e| e.id);
		}

		PgSchema::update(connection, LocationColumns::default(), |query| {
			query.push_values(entities_collected.iter(), |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(e.outer.as_ref().map(|o| o.id));
			});
		})
		.await
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_adapter::{schema::LocationAdapter, Updatable};
	use clinvoice_match::MatchLocation;
	use clinvoice_schema::Location;
	use pretty_assertions::{assert_eq, assert_ne};

	use crate::schema::{util, PgLocation};

	#[tokio::test]
	async fn update()
	{
		let connection = util::connect().await;

		let (mut earth, mars) = futures::try_join!(
			PgLocation::create(&connection, "Earth".into(), None),
			PgLocation::create(&connection, "Mars".into(), None),
		)
		.unwrap();

		let (mut chile, mut usa) = futures::try_join!(
			PgLocation::create(&connection, "Chile".into(), Some(earth.clone())),
			PgLocation::create(&connection, "USA".into(), Some(earth.clone())),
		)
		.unwrap();

		chile.name = "Chilé".into();
		chile.outer = Some(
			Location {
				id: earth.id,
				name: format!("Not {}", &earth.name),
				outer: None,
			}
			.into(),
		);
		earth.name = "Urth".into();

		usa.outer = Some(mars.into());

		{
			let mut transaction = connection.begin().await.unwrap();
			PgLocation::update(&mut transaction, [&chile, &usa, &earth].into_iter())
				.await
				.unwrap();
			transaction.commit().await.unwrap();
		}

		let chile_db = PgLocation::retrieve(&connection, &MatchLocation {
			id: chile.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.pop()
		.unwrap();

		let usa_db = PgLocation::retrieve(&connection, &MatchLocation {
			id: usa.id.into(),
			..Default::default()
		})
		.await
		.unwrap()
		.pop()
		.unwrap();

		assert_eq!(chile.id, chile_db.id);
		assert_eq!(chile.name, chile_db.name);
		assert_ne!(chile.outer, chile_db.outer);
		assert_eq!(earth, *chile_db.outer.unwrap());

		assert_eq!(usa, usa_db);
	}
}
