use super::TomlLocation;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, LocationAdapter}, Store};
use clinvoice_data::Id;
use std::error::Error;

impl<'name, 'pass, 'path, 'user> LocationAdapter<'name, 'pass, 'path, 'user>
for TomlLocation<'name, 'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Create a new `Location` with a generated ID.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */};
	/// ```
	fn create(name: &str, store: Store<'pass, 'path, 'user>) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Create a new [`Location`] which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// ```ignore
	/// Location {name, id: /* generated */, outside_id: self.unroll().id};
	/// ```
	fn create_inner(&self, name: &str) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&TomlLocation::path(store))?;
		return Ok(());
	}

	/// # Summary
	///
	/// Retrieve a [`Location`] from an active [`Store`](core::Store).
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// * An [`Error`], when something goes wrong.
	/// * A list of matches, if there are any.
	fn retrieve<'arr>(
		id: AnyValue<Id>,
		name: AnyValue<&str>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<&'arr [Self], Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{LocationAdapter, TomlLocation, util};
	use std::{fs, io};

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(TomlLocation::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = TomlLocation::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(TomlLocation::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());
		});
	}
}
