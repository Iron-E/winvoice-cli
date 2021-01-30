use super::BincodeEmployee;
use crate::util;
use clinvoice_adapter::{data::{EmployeeAdapter, RetrieveWhen, Updatable}, Store};
use clinvoice_data::{Contact, Employee, Organization, Person, Id};
use std::{collections::HashSet, error::Error};

impl<'email, 'name, 'pass, 'path, 'phone, 'title, 'user> EmployeeAdapter<'email, 'name, 'pass, 'path, 'phone, 'title, 'user>
for BincodeEmployee<'email, 'phone, 'title, 'pass, 'path, 'user>
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
	fn create(
		contact_info: HashSet<Contact<'email, 'phone>>,
		organization: Organization<'name>,
		person: Person<'email, 'name, 'phone>,
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
				id: util::unique_id(&Self::path(&store))?,
				organization_id: organization.id,
				person_id: person.id,
				title,
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
		contact_info: RetrieveWhen<Contact<'email, 'phone>>,
		id: RetrieveWhen<Id>,
		organization: RetrieveWhen<Organization<'name>>,
		person: RetrieveWhen<Person<'email, 'name, 'phone>>,
		store: Store<'pass, 'path, 'user>,
		title: RetrieveWhen<&'title str>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodeEmployee, Contact, EmployeeAdapter, HashSet, Id, Organization, Person, util};
	use std::{fs, io, time::Instant};

	#[test]
	fn test_create() -> Result<(), io::Error>
	{
		fn assertion(bincode_employee: BincodeEmployee<'_, '_, '_, '_, '_, '_>)
		{
			let read_result = fs::read(bincode_employee.filepath()).unwrap();

			assert_eq!(bincode_employee.employee, bincode::deserialize(&read_result).unwrap());
		}

		let start = Instant::now();

		let organization = Organization
		{
			id: Id::new_v4(),
			location_id: Id::new_v4(),
			name: "Big Old Test Corporation",
			representatives: HashSet::new(),
		};

		return util::test_temp_store(|store|
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
					name: "Testy Mćtesterson",
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
					name: "Nimron MacBeaver",
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
					name: "An Actual «Tor♯tust",
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
					name: "Jimmy Neutron, Boy Genius' Dog 'Gottard'",
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
					name: "Testy Mćtesterson",
				},
				*store,
				"Lazy No-good Duplicate Name User",
			).unwrap());

			assert!(fs::remove_dir_all(BincodeEmployee::path(&store)).is_ok());

			println!("\n>>>>> BincodeEmployee test_create {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		let start = Instant::now();

		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(BincodeEmployee::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = BincodeEmployee::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(BincodeEmployee::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());

			println!("\n>>>>> BincodeEmployee test_init {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
		});
	}
}
