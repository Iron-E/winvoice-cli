use
{
	super::BincodePerson,
	crate::util,
	clinvoice_adapter::
	{
		data::{MatchWhen, PersonAdapter, Updatable},
		Store
	},
	clinvoice_data::{Contact, Person, Id},
	std::{collections::HashSet, error::Error, fs, io::BufReader},
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
		contact_info: HashSet<Contact>,
		name: &'name str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>
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
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&Self::path(store))?;
		return Ok(());
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
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		let mut results = HashSet::new();

		for node_path in util::read_files(BincodePerson::path(&store))?
		{
			let person: Person = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if contact_info.set_matches(&person.contact_info) &&
				id.is_match(&person.id) &&
				name.is_match(&person.name)
			{
				results.insert(BincodePerson {person, store});
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
		super::{BincodePerson, Contact, HashSet, Id, MatchWhen, PersonAdapter, util},
		std::{fs, time::Instant},
		bincode,
	};

	#[test]
	fn test_create()
	{
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let mut contact_info = HashSet::new();

			contact_info.insert(Contact::Address(Id::new_v4()));
			test_create_assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Email("foo@bar.io".into()));
			test_create_assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Phone("1-800-555-3600".into()));
			test_create_assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Address(Id::new_v4()));
			test_create_assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Email("obviousemail@server.com".into()));
			test_create_assertion(BincodePerson::create(contact_info, "", *store).unwrap());

			println!("\n>>>>> BincodePerson test_create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
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
		let start = Instant::now();

		util::test_temp_store(|store|
		{
			let flingo = BincodePerson::create(
				[Contact::Address(Id::new_v4())].iter().cloned().collect(),
				"flingo",
				*store
			).unwrap();

			let bob = BincodePerson::create(
				[Contact::Email("foo@bar.io".into())].iter().cloned().collect(),
				"bob",
				*store
			).unwrap();

			let slimdi = BincodePerson::create(
				[Contact::Phone("1-800-555-3600".into())].iter().cloned().collect(),
				"slimdi",
				*store
			).unwrap();

			let longone = BincodePerson::create(
				[Contact::Address(Id::new_v4())].iter().cloned().collect(),
				"longone",
				*store
			).unwrap();

			// Retrieve bob
			let mut results = BincodePerson::retrieve(
				MatchWhen::HasAll(bob.person.contact_info.clone()), // contact info
				MatchWhen::Any, // id
				MatchWhen::Any, // name
				*store,
			).unwrap();

			// Assert bob is the only one retrieved
			assert!(!results.contains(&flingo));
			assert!(results.contains(&bob));
			assert!(!results.contains(&slimdi));
			assert!(!results.contains(&longone));

			// Retrieve longone and slimdi
			results = BincodePerson::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::HasAny([slimdi.person.name.clone(), longone.person.name.clone()].iter().cloned().collect()), // name
				*store,
			).unwrap();

			// Assert bob is the only one retrieved
			assert!(!results.contains(&flingo));
			assert!(!results.contains(&bob));
			assert!(results.contains(&slimdi));
			assert!(results.contains(&longone));

			println!("\n>>>>> BincodePerson test_retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
