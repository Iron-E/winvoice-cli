use super::{AnyValue, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{Contact, Id, Person};
use std::error::Error;

pub trait PersonAdapter<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'user> :
	Deletable<'pass, 'path, 'user> +
	Into<Person<'contact_info, 'email, 'name, 'phone>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	fn create<'err>(contact_info: &'contact_info [Contact<'email, 'phone>], name: &'name str) -> Result<Self, &'err dyn Error>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init<'err>(store: Store<'pass, 'path, 'user>) -> Result<(), &'err dyn Error>;

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
	) -> Result<Option<&'arr [Self]>, &'err dyn Error>;
}
