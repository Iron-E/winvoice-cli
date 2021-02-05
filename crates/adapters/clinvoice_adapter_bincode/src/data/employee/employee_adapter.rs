use
{
	super::BincodeEmployee,
	crate::util,
	clinvoice_adapter::
	{
		data::{EmployeeAdapter, MatchWhen, Updatable},
		Store
	},
	clinvoice_data::{Contact, Employee, Organization, Person, Id},
	std::{collections::HashSet, error::Error, fs, io::BufReader},
};

impl<'pass, 'path, 'user> EmployeeAdapter<'pass, 'path, 'user> for BincodeEmployee<'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Create some [`Employee`] on an active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * The created [`Employee`], if there were no errors.
	/// * An [`Error`], if something goes wrong.
	fn create<'title>(
		contact_info: HashSet<Contact>,
		organization: Organization,
		person: Person,
		store: Store<'pass, 'path, 'user>,
		title: &'title str,
	) -> Result<Self, Box<dyn Error>>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			employee: Employee
			{
				contact_info,
				employed: true,
				id: util::unique_id(&Self::path(&store))?,
				organization_id: organization.id,
				person_id: person.id,
				title: title.into(),
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
	/// Retrieve some [`Employee`] from an active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Employee`].
	///
	/// # Returns
	///
	/// * Any matching [`Employee`]s.
	/// * An [`Error`], should something go wrong.
	fn retrieve(
		contact_info: MatchWhen<Contact>,
		employed: MatchWhen<bool>,
		id: MatchWhen<Id>,
		organization: MatchWhen<Id>,
		person: MatchWhen<Id>,
		title: MatchWhen<String>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		let mut results = HashSet::new();

		for node_path in fs::read_dir(BincodeEmployee::path(&store))?.filter_map(
			|node| match node {Ok(n) => Some(n.path()), Err(_) => None}
		)
		{
			let employee: Employee = bincode::deserialize_from(
				BufReader::new(fs::File::open(node_path)?
			))?;

			if contact_info.set_matches(&employee.contact_info) &&
				employed.is_match(&employee.employed) &&
				id.is_match(&employee.id) &&
				organization.is_match(&employee.organization_id) &&
				person.is_match(&employee.person_id) &&
				title.is_match(&employee.title)
			{
				results.insert(BincodeEmployee {employee, store});
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
		super::{BincodeEmployee, Contact, EmployeeAdapter, HashSet, Id, MatchWhen, Organization, Person, util},
		core::hash::Hash,
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		fn assertion(bincode_employee: BincodeEmployee<'_, '_, '_>)
		{
			let read_result = fs::read(bincode_employee.filepath()).unwrap();
			assert_eq!(bincode_employee.employee, bincode::deserialize(&read_result).unwrap());
		}

		let start = Instant::now();

		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
			representatives: HashSet::new(),
		};

		util::test_temp_store(|store|
		{
			let mut contact_info = HashSet::new();

			contact_info.insert(Contact::Address(Id::new_v4()));
			assertion(BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				*store,
				"CEO of Tests",
			).unwrap());

			contact_info.insert(Contact::Email("foo@bar.io".into()));
			assertion(BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Nimron MacBeaver".into(),
				},
				*store,
				"Oblong Shape Holder",
			).unwrap());

			contact_info.insert(Contact::Phone("1-800-555-3600".into()));
			assertion(BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "An Actual «Tor♯tust".into(),
				},
				*store,
				"Mixer of Soups",
			).unwrap());

			contact_info.insert(Contact::Address(Id::new_v4()));
			assertion(BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
				},
				*store,
				"Sidekick",
			).unwrap());

			contact_info.insert(Contact::Email("obviousemail@server.com".into()));
			assertion(BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				*store,
				"Lazy No-good Duplicate Name User",
			).unwrap());

			println!("\n>>>>> BincodeEmployee test_create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	#[test]
	fn test_retrieve()
	{
		fn to_hashset<T>(slice: &[T]) -> HashSet<T> where T : Clone + Eq + Hash
		{
			return slice.into_iter().cloned().collect();
		}

		let start = Instant::now();

		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
			representatives: HashSet::new(),
		};

		util::test_temp_store(|store|
		{
			let mut contact_info = HashSet::new();

			contact_info.insert(Contact::Address(Id::new_v4()));
			let testy_mctesterson = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				*store,
				"CEO of Tests",
			).unwrap();

			contact_info.insert(Contact::Email("foo@bar.io".into()));
			let nimron_macbeaver = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Nimron MacBeaver".into(),
				},
				*store,
				"Oblong Shape Holder",
			).unwrap();

			contact_info.insert(Contact::Phone("1-800-555-3600".into()));
			let an_actual_tortust = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "An Actual «Tor♯tust".into(),
				},
				*store,
				"Mixer of Soups",
			).unwrap();

			contact_info.insert(Contact::Address(Id::new_v4()));
			let gottard = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
				},
				*store,
				"Sidekick",
			).unwrap();

			contact_info.insert(Contact::Email("obviousemail@server.com".into()));
			let duplicate_name = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				*store,
				"Lazy No-good Duplicate Name User",
			).unwrap();


			// Retrieve everything.
			let mut results = BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // employed
				MatchWhen::Any, // id
				MatchWhen::Any, // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				*store,
			).unwrap();

			// Assert the results contains all values
			assert!(results.contains(&testy_mctesterson));
			assert!(results.contains(&nimron_macbeaver));
			assert!(results.contains(&an_actual_tortust));
			assert!(results.contains(&gottard));
			assert!(results.contains(&duplicate_name));

			// Retrieve Arizona
			results = BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // employed
				MatchWhen::HasAny(to_hashset(&[testy_mctesterson.employee.id, gottard.employee.id])), // id
				MatchWhen::Any, // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				*store,
			).unwrap();

			// Assert the results contains all values
			assert!(results.contains(&testy_mctesterson));
			assert!(!results.contains(&nimron_macbeaver));
			assert!(!results.contains(&an_actual_tortust));
			assert!(results.contains(&gottard));
			assert!(!results.contains(&duplicate_name));

			println!("\n>>>>> BincodeEmployee test_retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
