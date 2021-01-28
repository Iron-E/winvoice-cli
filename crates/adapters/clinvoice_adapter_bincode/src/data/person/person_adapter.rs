use super::BincodePerson;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, PersonAdapter, Updatable}, Store};
use clinvoice_data::{Contact, Id, Person};
use std::{collections::BTreeSet, error::Error};

impl<'email, 'name, 'pass, 'path, 'phone, 'user> PersonAdapter<'email, 'name, 'pass, 'path, 'phone, 'user>
for BincodePerson<'email, 'name, 'phone, 'pass, 'path, 'user>
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
	fn create(
		contact_info: BTreeSet<Contact<'email, 'phone>>,
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
				id: util::next_id(&Self::path(&store))?,
				name,
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
		contact_info: AnyValue<BTreeSet<Contact<'email, 'phone>>>,
		id: AnyValue<Id>,
		name: AnyValue<&'name str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<BTreeSet<Self>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{BincodePerson, BTreeSet, Contact, PersonAdapter, util};
	use std::{fs, io};
	use bincode;

	#[test]
	fn test_create() -> Result<(), io::Error>
	{
		fn assertion(bincode_person: BincodePerson<'_, '_, '_, '_, '_, '_>)
		{
			let read_result = fs::read(bincode_person.filepath()).unwrap();

			assert_eq!(bincode_person.person, bincode::deserialize(&read_result).unwrap());
		}

		return util::test_temp_store(|store|
		{
			let mut contact_info = BTreeSet::new();

			contact_info.insert(Contact::Address(0));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Email("foo@bar.io".into()));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Phone("1-800-555-3600".into()));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Address(1));
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());

			contact_info.insert(Contact::Email("obviousemail@server.com".into()));
			assertion(BincodePerson::create(contact_info, "", *store).unwrap());

			assert!(fs::remove_dir_all(BincodePerson::path(&store)).is_ok());
		});
	}

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(BincodePerson::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = BincodePerson::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(BincodePerson::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());
		});
	}
}