use super::{PATH, TomlEmployee};
use crate::util;
use clinvoice_adapter::{data::{AnyValue, EmployeeAdapter}, Store};
use clinvoice_data::{Contact, Id, Organization, Person};
use std::error::Error;

impl<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'title, 'user> EmployeeAdapter<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'title, 'user>
for TomlEmployee<'contact_info, 'email, 'phone, 'title, 'pass, 'path, 'user>
where
	'email : 'contact_info,
	'phone : 'contact_info,
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
		contact_info: &'contact_info [Contact<'email, 'phone>],
		organization: Organization<'name>,
		person: Person<'contact_info, 'email, 'name, 'phone>,
		store: Store<'pass, 'path, 'user>,
		title: &'title str,
	) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		return util::create_store_dir(store, PATH);
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
	fn retrieve<'arr>(
		contact_info: AnyValue<&'contact_info [Contact<'email, 'phone>]>,
		id: AnyValue<Id>,
		organization: AnyValue<Organization<'name>>,
		person: AnyValue<Person<'contact_info, 'email, 'name, 'phone>>,
		store: Store<'pass, 'path, 'user>,
		title: AnyValue<&'title str>,
	) -> Result<&'arr [Self], Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{PATH, EmployeeAdapter, TomlEmployee, util};
	use std::{fs, io, path::Path};

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		return util::test_temp_store(
			|store|
			{
				// Assert that the function can initialize the store.
				assert!(TomlEmployee::init(&store).is_ok());

				// Create filepath for temporary test file.
				let filepath = Path::new(&store.path).join(PATH).join("testfile.txt");

				// Assert that creation of a file inside the initialized space is done
				assert!(fs::write(&filepath, "").is_ok());

				// Assert that the function won't re-initialize the store if it isn't empty.
				assert!(TomlEmployee::init(&store).is_err());

				// Assert cleanup
				assert!(fs::remove_file(&filepath).is_ok());
			},
		);
	}
}
