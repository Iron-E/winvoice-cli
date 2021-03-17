use
{
	super::BincodePerson,
	crate::
	{
		data::{Error, Result},
		util,
	},
	clinvoice_adapter::
	{
		data::{MatchWhen, Initializable, PersonAdapter, Updatable},
		Store,
	},
	clinvoice_data::{Contact, Person, Id},
	std::{collections::HashMap, fs, io::BufReader},
};

impl<'store> PersonAdapter<'store> for BincodePerson<'_, 'store>
{
	type Error = Error;

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
	fn create(contact_info: HashMap<String, Contact>, name: &str, store: &'store Store,) -> Result<Person>
	{
		Self::init(&store)?;

		let person = Person
		{
			contact_info,
			id: util::unique_id(&Self::path(&store))?,
			name: name.into(),
		};

		BincodePerson {person: &person, store}.update()?;

		Ok(person)
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
		store: &Store,
	) -> Result<Vec<Person>>
	{
		Self::init(&store)?;

		let mut results = Vec::new();

		for node_path in util::read_files(BincodePerson::path(&store))?
		{
			let person: Person = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if contact_info.set_matches(&person.contact_info.iter().map(|(_, v)| v).collect()) &&
				id.is_match(&person.id) &&
				name.is_match(&person.name)
			{
				results.push(person);
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
		super::{BincodePerson, Contact, Id, MatchWhen, Person, PersonAdapter, Store, util},
		std::{fs, time::Instant},
		bincode,
	};

	#[test]
	fn test_create()
	{
		util::test_temp_store(|store|
		{
			let start = Instant::now();

			test_create_assertion(
				BincodePerson::create(
					vec![("Home Address".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
					"Widdle",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodePerson::create(
					vec![("Personal Email".into(), Contact::Email("foo@bar.io".into()))].into_iter().collect(),
					"Long",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodePerson::create(
					vec![("Home Number".into(), Contact::Phone("1-800-555-3600".into()))].into_iter().collect(),
					"Steven",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodePerson::create(
					vec![("PO Box".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
					"JingleBob",
					&store,
				).unwrap(),
				&store,
			);

			test_create_assertion(
				BincodePerson::create(
					vec![("Throwaray Email".into(), Contact::Email("obviousemail@server.com".into()))].into_iter().collect(),
					"asldkj jdsoai",
					&store,
				).unwrap(),
				&store,
			);

			println!("\n>>>>> BincodePerson::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn test_create_assertion(person: Person, store: &Store)
	{
		let read_result = fs::read(BincodePerson {person: &person, store}.filepath()).unwrap();
		assert_eq!(person, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		util::test_temp_store(|store|
		{
			let flingo = BincodePerson::create(
				vec![("Home Address".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
				"flingo",
				&store
			).unwrap();

			let bob = BincodePerson::create(
				vec![("Personal Email".into(), Contact::Email("foo@bar.io".into()))].into_iter().collect(),
				"bob",
				&store
			).unwrap();

			let slimdi = BincodePerson::create(
				vec![("Home Phone".into(), Contact::Phone("1-800-555-3600".into()))].into_iter().collect(),
				"slimdi",
				&store
			).unwrap();

			let longone = BincodePerson::create(
				vec![("PO Box".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
				"longone",
				&store
			).unwrap();

			let start = Instant::now();

			// Retrieve bob
			let only_bob = BincodePerson::retrieve(
				MatchWhen::HasAll(bob.contact_info.iter().map(|(_, v)| v).collect()), // contact info
				MatchWhen::Any, // id
				MatchWhen::Any, // name
				&store,
			).unwrap();

			// Retrieve longone and slimdi
			let longone_slimdi = BincodePerson::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::HasAny([slimdi.name.clone(), longone.name.clone()].iter().collect()), // name
				&store,
			).unwrap();

			println!("\n>>>>> BincodePerson::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

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
