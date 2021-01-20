use super::{PATH, TomlOrganization};
use crate::util;
use clinvoice_adapter::{data::{AnyValue, OrganizationAdapter}, Store};
use clinvoice_data::{Employee, Id, Location};
use std::{collections::HashSet, error::Error};

impl<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'title, 'user> OrganizationAdapter<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'title, 'user>
for TomlOrganization<'name, 'pass, 'path, 'user>
where
	'email : 'contact_info,
	'phone : 'contact_info,
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
	fn retrieve<'arr>(
		id: AnyValue<Id>,
		location: AnyValue<Location<'name>>,
		name: AnyValue<&'name str>,
		representatives: AnyValue<HashSet<Employee>>,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Option<&'arr [Self]>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{Store, OrganizationAdapter, TomlOrganization};
	use clinvoice_adapter::Adapters;
	use std::{env, io};

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		let temp_path = env::temp_dir().join("clinvoice_adapter_toml_test_init");

		let test_store = Store
		{
			adapter: Adapters::TOML,
			password: None,
			path: match temp_path.to_str()
			{
				Some(s) => s,
				None => return Err(io::Error::new(
					io::ErrorKind::InvalidInput,
					"`env::temp_path` did not resolve to a valid path."
				)),
			},
			username: None,
		};

		assert!(TomlOrganization::init(&test_store).is_ok());
		assert!(TomlOrganization::init(&test_store).is_err());

		return Ok(());
	}
}
