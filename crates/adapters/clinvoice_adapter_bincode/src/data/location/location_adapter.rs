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
		data::{Initializable, LocationAdapter, MatchWhen, Updatable},
		Store,
	},
	clinvoice_data::{Location, Id},
	std::{fs, io::BufReader},
};

impl<'store> LocationAdapter<'store> for BincodeLocation<'store>
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
	fn create(name: &str, store: &'store Store) -> Result<Location>
	{
		Self::init(&store)?;

		let bincode_location = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&store))?,
				name: name.into(),
				outer_id: None,
			},
			store,
		};

		bincode_location.update()?;

		Ok(bincode_location.location)
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
	fn create_inner(&self, name: &str) -> Result<Location>
	{
		let inner_person = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&self.store))?,
				name: name.into(),
				outer_id: Some(self.location.id),
			},
			store: self.store,
		};

		inner_person.update()?;

		Ok(inner_person.location)
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
	fn retrieve(
		id: MatchWhen<Id>,
		name: MatchWhen<String>,
		outer: MatchWhen<Option<Id>>,
		store: &'store Store,
	) -> Result<Vec<Location>>
	{
		Self::init(&store)?;

		let mut results = Vec::new();

		for node_path in util::read_files(BincodeLocation::path(&store))?
		{
			let location: Location = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if id.is_match(&location.id) &&
				name.is_match(&location.name) &&
				outer.is_match(&location.outer_id)
			{
				results.push(location);
			}
		}

		Ok(results)
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, Id, Location, LocationAdapter, MatchWhen, Store, util},
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		util::test_temp_store(|store|
		{
			let start = Instant::now();
			let earth = BincodeLocation::create("Earth", &store).unwrap();
			let usa = BincodeLocation {location: earth, store}.create_inner("USA").unwrap();
			let arizona = BincodeLocation {location: usa, store}.create_inner("Arizona").unwrap();
			let phoenix = BincodeLocation {location: arizona, store}.create_inner("Phoenix").unwrap();
			println!("\n>>>>> BincodeLocation::start {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 4);

			assert_eq!(usa.outer_id, Some(earth.id));
			assert_eq!(arizona.outer_id, Some(usa.id));
			assert_eq!(phoenix.outer_id, Some(arizona.id));
			test_create_assertion(earth, &store);
			test_create_assertion(usa, &store);
			test_create_assertion(arizona, &store);
			test_create_assertion(phoenix, &store);
		});
	}

	/// The assertion most commonly used for the [`create` test](test_create).
	fn test_create_assertion(location: Location, store: &Store)
	{
		let read_result = fs::read(BincodeLocation {location, store}.filepath()).unwrap();
		assert_eq!(location, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", &store).unwrap();
			let usa = BincodeLocation {location: earth, store}.create_inner("USA").unwrap();
			let arizona = BincodeLocation {location: usa, store}.create_inner("Arizona").unwrap();
			let phoenix = BincodeLocation {location: arizona, store}.create_inner("Phoenix").unwrap();

			let start = Instant::now();

			// Retrieve everything.
			let everything = BincodeLocation::retrieve(
				MatchWhen::Any, // id
				MatchWhen::Any, // name
				MatchWhen::Any, // outer id
				&store,
			).unwrap();

			// Retrieve Arizona
			let only_arizona = BincodeLocation::retrieve(
				MatchWhen::HasAny([earth.id, arizona.id].iter().collect()), // id
				MatchWhen::Any, // name
				MatchWhen::HasNone([Option::<Id>::None].iter().collect()), // outer id
				&store,
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
		})
	}
}
