use
{
	super::BincodePerson,
	crate::util,
	clinvoice_adapter::
	{
		DynamicResult,
		data::{MatchWhen, Initializable, PersonAdapter, Updatable},
		Store,
	},
	clinvoice_data::{Contact, Person, Id},
	std::{fs, io::BufReader},
};

impl<'pass, 'path, 'user> PersonAdapter<'pass, 'path, 'user> for BincodePerson<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	fn create<'name>(
		contact_info: Vec<Contact>,
		name: &'name str,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Self>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			person: Person
			{
				contact_info,
				id: util::unique_id(&Self::path(&store))?,
				name: name.into(),
			},
			store,
		};

		bincode_person.update()?;

		return Ok(bincode_person);
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(
		contact_info: MatchWhen<Contact>,
		id: MatchWhen<Id>,
		name: MatchWhen<String>,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Vec<Self>>
	{
		let mut results = Vec::new();

		for node_path in util::read_files(BincodePerson::path(&store))?
		{
			let person: Person = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if contact_info.set_matches(&person.contact_info.iter().cloned().collect()) &&
				id.is_match(&person.id) &&
				name.is_match(&person.name)
			{
				results.push(BincodePerson {person, store});
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
		super::{BincodePerson, Contact, Id, MatchWhen, PersonAdapter, util},
		std::{fs, time::Instant},
		bincode,
	};

	#[test]
	fn test_create()
	{
		util::test_temp_store(|store|
		{
			let start = Instant::now();
			test_create_assertion(BincodePerson::create(vec![Contact::Address(Id::new_v4())], "", *store).unwrap());
			test_create_assertion(BincodePerson::create(vec![Contact::Email("foo@bar.io".into())], "", *store).unwrap());
			test_create_assertion(BincodePerson::create(vec![Contact::Phone("1-800-555-3600".into())], "", *store).unwrap());
			test_create_assertion(BincodePerson::create(vec![Contact::Address(Id::new_v4())], "", *store).unwrap());
			test_create_assertion(BincodePerson::create(vec![Contact::Email("obviousemail@server.com".into())], "", *store).unwrap());
			println!("\n>>>>> BincodePerson::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	fn test_create_assertion(bincode_person: BincodePerson<'_, '_, '_>)
	{
		let read_result = fs::read(bincode_person.filepath()).unwrap();
		assert_eq!(bincode_person.person, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		util::test_temp_store(|store|
		{
			let flingo = BincodePerson::create(
				vec![Contact::Address(Id::new_v4())],
				"flingo",
				*store
			).unwrap();

			let bob = BincodePerson::create(
				vec![Contact::Email("foo@bar.io".into())],
				"bob",
				*store
			).unwrap();

			let slimdi = BincodePerson::create(
				vec![Contact::Phone("1-800-555-3600".into())],
				"slimdi",
				*store
			).unwrap();

			let longone = BincodePerson::create(
				vec![Contact::Address(Id::new_v4())],
				"longone",
				*store
			).unwrap();

			let start = Instant::now();

			// Retrieve bob
			let only_bob = BincodePerson::retrieve(
				MatchWhen::HasAll(bob.person.contact_info.iter().cloned().collect()), // contact info
				MatchWhen::Any, // id
				MatchWhen::Any, // name
				*store,
			).unwrap();

			// Retrieve longone and slimdi
			let longone_slimdi = BincodePerson::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::HasAny([slimdi.person.name.clone(), longone.person.name.clone()].iter().cloned().collect()), // name
				*store,
			).unwrap();

			println!("\n>>>>> BincodePerson::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

			// Assert bob is the only one retrieved
			assert!(!only_bob.contains(&flingo));
			assert!(only_bob.contains(&bob));
			assert!(!only_bob.contains(&slimdi));
			assert!(!only_bob.contains(&longone));

			// Assert bob is the only one retrieved
			assert!(!longone_slimdi.contains(&flingo));
			assert!(!longone_slimdi.contains(&bob));
			assert!(longone_slimdi.contains(&slimdi));
			assert!(longone_slimdi.contains(&longone));
		});
	}
}
