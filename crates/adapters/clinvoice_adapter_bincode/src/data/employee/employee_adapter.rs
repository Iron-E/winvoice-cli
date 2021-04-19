use
{
	std::collections::HashMap,

	super::BincodeEmployee,
	crate::
	{
		data::{Error, Result},
		util,
	},

	clinvoice_adapter::
	{
		data::{EmployeeAdapter, Error as DataError, Initializable, query, Updatable},
		Store,
	},
	clinvoice_data::{Contact, Employee, EmployeeStatus, Organization, Person},
};

impl EmployeeAdapter for BincodeEmployee<'_, '_>
{
	type Error = Error;

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
	fn create(
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: &str,
		store: &Store,
	) -> Result<Employee>
	{
		Self::init(&store)?;

		let employee = Employee
		{
			contact_info,
			id: util::unique_id(&Self::path(&store))?,
			organization_id: organization.id,
			person_id: person.id,
			title: title.into(),
			status,
		};

		BincodeEmployee {employee: &employee, store}.update()?;

		Ok(employee)
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
	fn retrieve(query: &query::Employee, store: &Store) -> Result<Vec<Employee>>
	{
		Self::init(&store)?;

		util::retrieve(Self::path(store), |e| query.matches(e).map_err(|e| DataError::from(e).into()))
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{borrow::Cow, fs, time::Instant},

		super::{BincodeEmployee, Contact, Employee, EmployeeAdapter, EmployeeStatus, Organization, Person, query, Store, util},

		clinvoice_adapter::data::query::Match,
		clinvoice_data::Id,
	};

	#[test]
	fn create()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::temp_store(|store|
		{
			let start = Instant::now();

			create_assertion(
				BincodeEmployee::create(
					vec![("Work".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Testy Mćtesterson".into(),
					},
					EmployeeStatus::Employed,
					"CEO of Tests",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeEmployee::create(
					vec![("Work Email".into(), Contact::Email("foo@bar.io".into()))].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Nimron MacBeaver".into(),
					},
					EmployeeStatus::NotEmployed,
					"Oblong Shape Holder",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeEmployee::create(
					vec![("Work Phone".into(), Contact::Phone("1-800-555-3600".into()))].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "An Actual «Tor♯tust".into(),
					},
					EmployeeStatus::Representative,
					"Mixer of Soups",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeEmployee::create(
					vec![("Work".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
					},
					EmployeeStatus::Employed,
					"Sidekick",
					&store,
				).unwrap(),
				&store,
			);

			create_assertion(
				BincodeEmployee::create(
					vec![("Work Email".into(), Contact::Email("obviousemail@server.com".into()))].into_iter().collect(),
					organization.clone(),
					Person
					{
						id: Id::new_v4(),
						name: "Testy Mćtesterson".into(),
					},
					EmployeeStatus::NotEmployed,
					"Lazy No-good Duplicate Name User",
					&store,
				).unwrap(),
				&store,
			);

			println!("\n>>>>> BincodeEmployee::create {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 5);
		});
	}

	fn create_assertion(employee: Employee, store: &Store)
	{
		let read_result = fs::read(BincodeEmployee {employee: &employee, store}.filepath()).unwrap();
		assert_eq!(employee, bincode::deserialize(&read_result).unwrap());
	}

	#[test]
	fn retrieve()
	{
		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation".into(),
		};

		util::temp_store(|store|
		{
			let testy_mctesterson = BincodeEmployee::create(
				vec![("Work Address".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
				organization.clone(),
				Person
				{
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				EmployeeStatus::NotEmployed,
				"CEO of Tests",
				&store,
			).unwrap();

			let nimron_macbeaver = BincodeEmployee::create(
				vec![("Home Address".into(), Contact::Email("foo@bar.io".into()))].into_iter().collect(),
				organization.clone(),
				Person
				{
					id: Id::new_v4(),
					name: "Nimron MacBeaver".into(),
				},
				EmployeeStatus::Employed,
				"Oblong Shape Holder",
				&store,
			).unwrap();

			let an_actual_tortust = BincodeEmployee::create(
				vec![("Work Phone".into(), Contact::Phone("1-800-555-3600".into()))].into_iter().collect(),
				organization.clone(),
				Person
				{
					id: Id::new_v4(),
					name: "An Actual «Tor♯tust".into(),
				},
				EmployeeStatus::Representative,
				"Mixer of Soups",
				&store,
			).unwrap();

			let gottard = BincodeEmployee::create(
				vec![("Work Address".into(), Contact::Address(Id::new_v4()))].into_iter().collect(),
				organization.clone(),
				Person
				{
					id: Id::new_v4(),
					name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'".into(),
				},
				EmployeeStatus::Employed,
				"Sidekick",
				&store,
			).unwrap();

			let duplicate_name = BincodeEmployee::create(
				vec![("Work Email".into(), Contact::Email("obviousemail@server.com".into()))].into_iter().collect(),
				organization.clone(),
				Person
				{
					id: Id::new_v4(),
					name: "Testy Mćtesterson".into(),
				},
				EmployeeStatus::NotEmployed,
				"Lazy No-good Duplicate Name User",
				&store,
			).unwrap();

			let start = Instant::now();

			let everything = BincodeEmployee::retrieve(&Default::default(), &store).unwrap();

			// Retrieve testy and gottard
			let testy_gottard = BincodeEmployee::retrieve(
				&query::Employee
				{
					id: Match::HasAny(vec![Cow::Borrowed(&testy_mctesterson.id), Cow::Borrowed(&gottard.id)].into_iter().collect()),
					..Default::default()
				},
				&store,
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
