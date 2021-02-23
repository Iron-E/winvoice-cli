use
{
	super::BincodeEmployee,
	crate::util,
	clinvoice_adapter::
	{
		data::{EmployeeAdapter, Initializable, MatchWhen, Updatable},
		DynamicResult, Store,
	},
	clinvoice_data::{Contact, Employee, EmployeeStatus, Organization, Person, Id},
	std::{fs, io::BufReader},
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
		contact_info: Vec<Contact>,
		organization: Organization,
		person: Person,
		title: &'title str,
		status: EmployeeStatus,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Self>
	{
		Self::init(&store)?;

		let bincode_person = Self
		{
			employee: Employee
			{
				contact_info,
				id: util::unique_id(&Self::path(&store))?,
				organization_id: organization.id,
				person_id: person.id,
				title: title.into(),
				status,
			},
			store,
		};

		bincode_person.update()?;

		return Ok(bincode_person);
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
		id: MatchWhen<Id>,
		organization: MatchWhen<Id>,
		person: MatchWhen<Id>,
		title: MatchWhen<String>,
		status: MatchWhen<EmployeeStatus>,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Vec<Self>>
	{
		let mut results = Vec::new();

		for node_path in util::read_files(BincodeEmployee::path(&store))?
		{
			let employee: Employee = bincode::deserialize_from(BufReader::new(
				fs::File::open(node_path)?
			))?;

			if contact_info.set_matches(&employee.contact_info.iter().cloned().collect()) &&
				id.is_match(&employee.id) &&
				organization.is_match(&employee.organization_id) &&
				person.is_match(&employee.person_id) &&
				title.is_match(&employee.title) &&
				status.is_match(&employee.status)
			{
				results.push(BincodeEmployee {employee, store});
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
		super::{BincodeEmployee, Contact, EmployeeAdapter, EmployeeStatus, Id, MatchWhen, Organization, Person, util},
		std::{fs, time::Instant},
	};

	#[test]
	fn test_create()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::test_temp_store(|store|
		{
			let start = Instant::now();

			test_create_assertion(BincodeEmployee::create(
				vec![Contact::Address(Id::new_v4())],
				organization.clone(),
				Person
				{
					contact_info: vec![Contact::Address(Id::new_v4())],
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"CEO of Tests",
				EmployeeStatus::Employed,
				*store,
			).unwrap());

			test_create_assertion(BincodeEmployee::create(
				vec![Contact::Email("foo@bar.io".into())],
				organization.clone(),
				Person
				{
					contact_info: vec![Contact::Email("foo2@bar.io".into())],
					id: Id::new_v4(),
					name: "Nimron MacBeaver".into(),
				},
				"Oblong Shape Holder",
				EmployeeStatus::NotEmployed,
				*store,
			).unwrap());

			test_create_assertion(BincodeEmployee::create(
				vec![Contact::Phone("1-800-555-3600".into())],
				organization.clone(),
				Person
				{
					contact_info: vec![Contact::Phone("1-800-555-3601".into())],
					id: Id::new_v4(),
					name: "An Actual «Tor♯tust".into(),
				},
				"Mixer of Soups",
				EmployeeStatus::Representative,
				*store,
			).unwrap());

			test_create_assertion(BincodeEmployee::create(
				vec![Contact::Address(Id::new_v4())],
				organization.clone(),
				Person
				{
					contact_info: vec![Contact::Address(Id::new_v4())],
					id: Id::new_v4(),
					name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
				},
				"Sidekick",
				EmployeeStatus::Employed,
				*store,
			).unwrap());

			test_create_assertion(BincodeEmployee::create(
				vec![Contact::Email("obviousemail@server.com".into())],
				organization.clone(),
				Person
				{
					contact_info: vec![Contact::Email("obviousemail2@server.com".into())],
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"Lazy No-good Duplicate Name User",
				EmployeeStatus::NotEmployed,
				*store,
			).unwrap());

			println!("\n>>>>> BincodeEmployee::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn test_create_assertion(bincode_employee: BincodeEmployee<'_, '_, '_>)
	{
		let read_result = fs::read(bincode_employee.filepath()).unwrap();
		assert_eq!(bincode_employee.employee, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn test_retrieve()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::test_temp_store(|store|
		{
			let mut contact_info = Vec::new();

			contact_info.push(Contact::Address(Id::new_v4()));
			let testy_mctesterson = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"CEO of Tests",
				EmployeeStatus::NotEmployed,
				*store,
			).unwrap();

			contact_info.push(Contact::Email("foo@bar.io".into()));
			let nimron_macbeaver = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Nimron MacBeaver".into(),
				},
				"Oblong Shape Holder",
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			contact_info.push(Contact::Phone("1-800-555-3600".into()));
			let an_actual_tortust = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "An Actual «Tor♯tust".into(),
				},
				"Mixer of Soups",
				EmployeeStatus::Representative,
				*store,
			).unwrap();

			contact_info.push(Contact::Address(Id::new_v4()));
			let gottard = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
				},
				"Sidekick",
				EmployeeStatus::Employed,
				*store,
			).unwrap();

			contact_info.push(Contact::Email("obviousemail@server.com".into()));
			let duplicate_name = BincodeEmployee::create(
				contact_info.clone(),
				organization.clone(),
				Person
				{
					contact_info: contact_info.clone(),
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				"Lazy No-good Duplicate Name User",
				EmployeeStatus::NotEmployed,
				*store,
			).unwrap();

			let start = Instant::now();

			let everything = BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::Any, // id
				MatchWhen::Any, // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				MatchWhen::Any, // status
				*store,
			).unwrap();

			// Retrieve testy and gottard
			let testy_gottard = BincodeEmployee::retrieve(
				MatchWhen::Any, // contact info
				MatchWhen::HasAny([testy_mctesterson.employee.id, gottard.employee.id].iter().cloned().collect()), // id
				MatchWhen::Any, // organization
				MatchWhen::Any, // person
				MatchWhen::Any, // title
				MatchWhen::Any, // status
				*store,
			).unwrap();

			println!("\n>>>>> BincodeEmployee::retrieve {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);

			// Assert the results contains all values
			assert!(everything.contains(&an_actual_tortust));
			assert!(everything.contains(&duplicate_name));
			assert!(everything.contains(&gottard));
			assert!(everything.contains(&nimron_macbeaver));
			assert!(everything.contains(&testy_mctesterson));

			// Assert the results contains all expected values
			assert!(!testy_gottard.contains(&an_actual_tortust));
			assert!(!testy_gottard.contains(&duplicate_name));
			assert!(testy_gottard.contains(&gottard));
			assert!(!testy_gottard.contains(&nimron_macbeaver));
			assert!(testy_gottard.contains(&testy_mctesterson));
		});
	}
}
