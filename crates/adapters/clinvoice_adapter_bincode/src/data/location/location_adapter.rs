use
{
	super::BincodeLocation,
	crate::util,
	clinvoice_adapter::
	{
		data::{Initializable, LocationAdapter, MatchWhen, Updatable},
		DynamicResult, Store,
	},
	clinvoice_data::{Location, Id},
	std::{collections::HashSet, fs, io::BufReader},
};

impl<'pass, 'path, 'user> LocationAdapter<'pass, 'path, 'user> for BincodeLocation<'pass, 'path, 'user>
{
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
	fn create<'name>(name: &'name str, store: Store<'pass, 'path, 'user>) -> DynamicResult<Self>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			location: Location
			{
				id: util::unique_id(&Self::path(&store))?,
				name: name.into(),
				outer_id: None,
			},
			store,
		};

		bincode_person.update()?;

		return Ok(bincode_person);
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
	fn create_inner<'name>(&self, name: &'name str) -> DynamicResult<Self>
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

		return Ok(inner_person);
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
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<HashSet<Self>>
	{
		let mut results = HashSet::new();

		for node_path in util::read_files(BincodeLocation::path(&store))?
		{
			let location: Location = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if id.is_match(&location.id) &&
				name.is_match(&location.name) &&
				outer.is_match(&location.outer_id)
			{
				results.insert(BincodeLocation {location, store});
			}
		}

		return Ok(results);
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{BincodeLocation, Id, LocationAdapter, MatchWhen, util},
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			test_create_assertion(&earth);

			let usa = earth.create_inner("USA").unwrap();
			assert_eq!(usa.location.outer_id, Some(earth.location.id));
			test_create_assertion(&usa);

			let arizona = usa.create_inner("Arizona").unwrap();
			assert_eq!(arizona.location.outer_id, Some(usa.location.id));
			test_create_assertion(&arizona);

			let phoenix = arizona.create_inner("Phoenix").unwrap();
			assert_eq!(phoenix.location.outer_id, Some(arizona.location.id));
			test_create_assertion(&phoenix);

			println!("\n>>>>> BincodeLocation test_start {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	/// The assertion most commonly used for the [`create` test](test_create).
	fn test_create_assertion(bincode_location: &BincodeLocation<'_, '_, '_>)
	{
		let read_result = fs::read(bincode_location.filepath()).unwrap();
		assert_eq!(bincode_location.location, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			let earth = BincodeLocation::create("Earth", *store).unwrap();
			let usa = earth.create_inner("USA").unwrap();
			let arizona = usa.create_inner("Arizona").unwrap();
			let phoenix = arizona.create_inner("Phoenix").unwrap();

			// Retrieve everything.
			let mut results = BincodeLocation::retrieve(
				MatchWhen::Any, // id
				MatchWhen::Any, // name
				MatchWhen::Any, // outer id
				*store,
			).unwrap();

			// Assert the results contains all values
			assert!(results.contains(&earth));
			assert!(results.contains(&usa));
			assert!(results.contains(&arizona));
			assert!(results.contains(&phoenix));

			// Retrieve Arizona
			results = BincodeLocation::retrieve(
				MatchWhen::HasAny([earth.location.id, arizona.location.id].iter().cloned().collect()), // id
				MatchWhen::Any, // name
				MatchWhen::HasNone([Option::<Id>::None].iter().cloned().collect()), // outer id
				*store,
			).unwrap();

			// Assert the results contains all values
			assert!(!results.contains(&earth));
			assert!(!results.contains(&usa));
			assert!(results.contains(&arizona));
			assert!(!results.contains(&phoenix));

			println!("\n>>>>> BincodeLocation test_retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
