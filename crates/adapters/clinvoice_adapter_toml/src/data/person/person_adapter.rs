use super::TomlPerson;
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
	fn create<'err>(contact_info: &'contact_info [Contact<'email, 'phone>], name: &'name str) -> Result<Self, &'err dyn Error>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init<'err>(store: Store<'pass, 'path, 'user>) -> Result<(), &'err dyn Error>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`]<'err>(crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve<'arr, 'err>(
		contact_info: AnyValue<&'contact_info [Contact<'email, 'phone>]>,
		id: AnyValue<Id>,
		name: AnyValue<&'name str>,
	) -> Result<Option<&'arr [Self]>, &'err dyn Error>
	{
		todo!()
	}
}

