use super::BincodeOrganization;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, OrganizationAdapter, Updatable}, Store};
use clinvoice_data::{Employee, Id, Location, Organization};
use std::{collections::HashSet, error::Error};

impl<'email, 'name, 'pass, 'path, 'phone, 'title, 'user> OrganizationAdapter<'email, 'name, 'pass, 'path, 'phone, 'title, 'user>
for BincodeOrganization<'name, 'pass, 'path, 'user>
{
	/// # Summary
	///
	/// Create a new [`Organization`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// The newly created [`Organization`].
	fn create(
		location: Location<'name>,
		name: &'name str,
		representatives: HashSet<Employee>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, Box<dyn Error>>
	{
		BincodeOrganization::init(&store)?;

		let bincode_organization = BincodeOrganization
		{
			organization: Organization
			{
				id: util::next_id(&BincodeOrganization::path(&store))?,
				location_id: location.id,
				name,
				representatives: representatives.iter().map(|rep| rep.id).collect(),
			},
			store,
		};

		bincode_organization.update()?;

		return Ok(bincode_organization);
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&BincodeOrganization::path(store))?;
		return Ok(());
	}

	/// # Summary
	///
	/// Retrieve some [`Organization`] from the active [`Store`]crate::Store).
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve(
		id: AnyValue<Id>,
		location: AnyValue<Location<'name>>,
		name: AnyValue<&'name str>,
		representatives: AnyValue<HashSet<Employee>>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<HashSet<Self>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{OrganizationAdapter, BincodeOrganization, util};
	use std::{fs, io};

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(BincodeOrganization::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = BincodeOrganization::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(BincodeOrganization::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());
		});
	}
}
