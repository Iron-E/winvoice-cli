use super::{AnyValue, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{Contact, Id, Person};
use std::{collections::BTreeSet, error::Error};

pub trait PersonAdapter<'email, 'name, 'pass, 'path, 'phone, 'user> :
	Deletable<'pass, 'path, 'user> +
	Into<Person<'email, 'name, 'phone>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;

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
	) -> Result<BTreeSet<Self>, Box<dyn Error>>;
}
