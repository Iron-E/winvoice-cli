use
{
	super::BincodeLocation,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, LocationAdapter, Updatable},
		Store,
	},
	clinvoice_data::Location,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl LocationAdapter for BincodeLocation<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */};
	/// ```
	async fn create(name: String, store: &Store) -> Result<Location>
	{
		let init_fut = Self::init(&store);

		let location = Location
		{
			id: util::unique_id(&Self::path(&store))?,
			name,
			outer_id: None,
		};

		init_fut.await?;
		BincodeLocation {location: &location, store}.update().await?;

		Ok(location)
	}

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */, outside_id: self.unroll().id};
	/// ```
	async fn create_inner(&self, name: String) -> Result<Location>
	{
		let inner_location = Location
		{
			id: util::unique_id(&Self::path(&self.store))?,
			name,
			outer_id: Some(self.location.id),
		};

		BincodeLocation {location: &inner_location, store: self.store}.update().await?;

		Ok(inner_location)
	}

	/// # Summary
	///
	/// Retrieve a [`Location`] from an active [`Store`](core::Store).
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// * An [`Error`], when something goes wrong.
	/// * A list of matches, if there are any.
	async fn retrieve(query: &query::Location, store: &Store) -> Result<Vec<Location>>
	{
		Self::init(&store).await?;

		util::retrieve(Self::path(store),
			|l| query.matches(l).map_err(|e| DataError::from(e).into()),
		).await
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{borrow::Cow::Borrowed, time::Instant},

		super::{BincodeLocation, Location, LocationAdapter, query, Store, util},

		clinvoice_query::Match,

		tokio::fs,
	};

	#[tokio::test]
	async fn create()
	{
		util::temp_store(|store| async move
		{
			let start = Instant::now();

			let earth = BincodeLocation::create("Earth".into(), &store).await.unwrap();
			let usa = BincodeLocation {location: &earth, store}.create_inner("USA".into()).await.unwrap();
			let arizona = BincodeLocation {location: &usa, store}.create_inner("Arizona".into()).await.unwrap();
			let phoenix = BincodeLocation {location: &arizona, store}.create_inner("Phoenix".into()).await.unwrap();

			println!("\n>>>>> BincodeLocation::start {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 4);

			assert_eq!(usa.outer_id, Some(earth.id));
			assert_eq!(arizona.outer_id, Some(usa.id));
			assert_eq!(phoenix.outer_id, Some(arizona.id));
			futures::join!(
				create_assertion(earth, &store),
				create_assertion(usa, &store),
				create_assertion(arizona, &store),
				create_assertion(phoenix, &store),
			);
		}).await;
	}

	/// The assertion most commonly used for the [`create` test](test_create).
	async fn create_assertion(location: Location, store: &Store)
	{
		let read_result = fs::read(BincodeLocation {location: &location, store}.filepath()).await.unwrap();
		assert_eq!(location, bincode::deserialize(&read_result).unwrap());
	}

	#[tokio::test]
	async fn retrieve()
	{
		util::temp_store(|store| async move
		{
			let earth = BincodeLocation::create("Earth".into(), &store).await.unwrap();
			let usa = BincodeLocation {location: &earth, store}.create_inner("USA".into()).await.unwrap();
			let arizona = BincodeLocation {location: &usa, store}.create_inner("Arizona".into()).await.unwrap();
			let phoenix = BincodeLocation {location: &arizona, store}.create_inner("Phoenix".into()).await.unwrap();

			let start = Instant::now();

			let (everything, only_arizona) = futures::try_join!(
				// Retrieve everything.
				BincodeLocation::retrieve(&Default::default(), &store),

				// Retrieve Arizona
				BincodeLocation::retrieve(
					&query::Location
					{
						id: Match::HasAny(vec![Borrowed(&earth.id), Borrowed(&arizona.id)].into_iter().collect()),
						outer: query::OuterLocation::Some(query::Location::default().into()),
						..Default::default()
					},
					&store,
				),
			).unwrap();

			println!("\n>>>>> BincodeLocation::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert the results contains all values
			assert!(everything.contains(&earth));
			assert!(everything.contains(&usa));
			assert!(everything.contains(&arizona));
			assert!(everything.contains(&phoenix));

			// Assert the results contains all values
			assert!(!only_arizona.contains(&earth));
			assert!(!only_arizona.contains(&usa));
			assert!(only_arizona.contains(&arizona));
			assert!(!only_arizona.contains(&phoenix));
		}).await
	}
}
