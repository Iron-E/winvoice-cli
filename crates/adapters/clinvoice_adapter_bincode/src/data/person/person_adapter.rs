use super::BincodePerson;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, PersonAdapter}, Store};
use clinvoice_data::{Contact, Id, Person};
use std::{collections::HashSet, error::Error, fs};
use bincode;

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
		contact_info: HashSet<Contact<'email, 'phone>>,
		name: &'name str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>
	{
		BincodePerson::init(&store)?;

		let bincode_person = BincodePerson
		{
			person: Person
			{
				contact_info,
				id: util::next_id(&BincodePerson::path(&store))?,
				name,
			},
			store,
		};

		fs::write(bincode_person.filepath(), bincode::serialize(&bincode_person.person)?)?;

		return Ok(bincode_person);
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&BincodePerson::path(store))?;
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
	fn retrieve<'arr>(
		contact_info: AnyValue<HashSet<Contact<'email, 'phone>>>,
		id: AnyValue<Id>,
		name: AnyValue<&'name str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Option<&'arr [Self]>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{Contact, HashSet, PersonAdapter, bincode, BincodePerson, util};
	use std::{fs, io};

	#[test]
	fn test_create() -> Result<(), io::Error>
	{
		fn assertion(toml_person: BincodePerson<'_, '_, '_, '_, '_, '_>)
		{
			let read_result = fs::read(toml_person.filepath()).unwrap();

			assert_eq!(toml_person.person, bincode::deserialize(&read_result).unwrap());
		}

		let mut contact_info = HashSet::new();
		contact_info.insert(Contact::Address(0));

		return util::test_temp_store(|store|
		{
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());
			assertion(BincodePerson::create(contact_info.clone(), "", *store).unwrap());
			assertion(BincodePerson::create(contact_info, "", *store).unwrap());

			// assert!(fs::remove_dir_all(BincodePerson::path(&store)).is_ok());
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
