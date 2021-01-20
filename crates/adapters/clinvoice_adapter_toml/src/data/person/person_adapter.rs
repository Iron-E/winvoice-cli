use super::{PATH, TomlPerson};
use crate::util;
use clinvoice_adapter::{data::{AnyValue, PersonAdapter}, Store};
use clinvoice_data::{Contact, Id};
use std::error::Error;

impl<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'user> PersonAdapter<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'user>
for TomlPerson<'contact_info, 'email, 'name, 'phone, 'pass, 'path, 'user>
where
	'email : 'contact_info,
	'phone : 'contact_info,
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
	fn create(contact_info: &'contact_info [Contact<'email, 'phone>], name: &'name str) -> Result<Self, Box<dyn Error>>
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
		contact_info: AnyValue<&'contact_info [Contact<'email, 'phone>]>,
		id: AnyValue<Id>,
		name: AnyValue<&'name str>,
	) -> Result<Option<&'arr [Self]>, Box<dyn Error>>
	{
		todo!()
	}
}

