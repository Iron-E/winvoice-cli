use super::{MatchWhen, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{Contact, Employee, Organization, Person, Id};
use core::ops::Deref;
use std::{collections::HashSet, error::Error};

pub trait EmployeeAdapter<'email, 'name, 'pass, 'path, 'phone, 'title, 'user> :
	Deletable<'pass, 'path, 'user> +
	Deref<Target=Employee<'email, 'phone, 'title>> +
	Into<Employee<'email, 'phone, 'title>> +
	Into<Result<Organization<'name>, Box<dyn Error>>> +
	Into<Result<Person<'email, 'name, 'phone>, Box<dyn Error>>> +
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
	fn create(
		contact_info: HashSet<Contact<'email, 'phone>>,
		organization: Organization<'name>,
		person: Person<'email, 'name, 'phone>,
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
		contact_info: MatchWhen<Contact<'email, 'phone>>,
		id: MatchWhen<Id>,
		organization: MatchWhen<Organization<'name>>,
		person: MatchWhen<Person<'email, 'name, 'phone>>,
		store: Store<'pass, 'path, 'user>,
		title: MatchWhen<&'title str>,
	) -> Result<HashSet<Self>, Box<dyn Error>>;
}
