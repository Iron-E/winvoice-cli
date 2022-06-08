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
		let mut peekable_entities = entities.clone().peekable();

		// There is nothing to do.
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();
		PgSchema::update(&mut *connection, COLUMNS, "locations", "L", "V", |query| {
			query.push_values(peekable_entities, |mut q, e| {
				q.push_bind(e.id)
					.push_bind(&e.name)
					.push_bind(e.outer.as_ref().map(|o| o.id));
			});
		})
		.await?;

		Self::update(connection, entities.filter_map(|e| e.outer.as_deref())).await?;

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
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn update()
	{
		let connection = util::connect().await;

		let earth = PgLocation::create(&connection, "Earth".into(), None)
			.await
			.unwrap();

		let usa = PgLocation::create(&connection, "USA".into(), Some(earth.clone()))
			.await
			.unwrap();

		let (mut arizona, mut utah) = futures::try_join!(
			PgLocation::create(&connection, "Arizona".into(), Some(usa.clone())),
			PgLocation::create(&connection, "Utah".into(), Some(usa.clone())),
		)
		.unwrap();

		arizona.name = "Not Arizona".into();
		utah.outer = utah.outer.map(|mut o| {
			o.name = "Not USA".into();
			o
		});

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
