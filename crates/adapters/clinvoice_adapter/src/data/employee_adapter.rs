use super::{MatchWhen, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{Contact, Employee, Organization, Person, Id};
use core::ops::Deref;
use std::{collections::HashSet, error::Error};

pub trait EmployeeAdapter<'pass, 'path, 'user> :
	Deletable +
	Deref<Target=Employee> +
	Into<Employee> +
	Into<Result<Organization, Box<dyn Error>>> +
	Into<Result<Person, Box<dyn Error>>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	fn create<'title>(
		contact_info: HashSet<Contact>,
		organization: Organization,
		person: Person,
		store: Store<'pass, 'path, 'user>,
		title: &'title str,
	) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;

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
	fn retrieve(
		contact_info: MatchWhen<Contact>,
		id: MatchWhen<Id>,
		organization: MatchWhen<Id>,
		person: MatchWhen<Id>,
		store: Store<'pass, 'path, 'user>,
		title: MatchWhen<String>,
	) -> Result<HashSet<Self>, Box<dyn Error>>;
}
