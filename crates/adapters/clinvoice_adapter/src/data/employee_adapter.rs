use super::{AnyValue, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{Contact, Employee, Id, Organization, Person};
use std::error::Error;

pub trait EmployeeAdapter<'contact_info, 'email, 'name, 'pass, 'path, 'phone, 'title, 'user> :
	Deletable<'pass, 'path, 'user> +
	Into<Employee<'contact_info, 'email, 'phone, 'title>> +
	Into<Result<Organization<'name>, Box<dyn Error>>> +
	Into<Result<Person<'contact_info, 'email, 'name, 'phone>, Box<dyn Error>>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	fn retrieve<'arr>(
		contact_info: AnyValue<&'contact_info [Contact<'email, 'phone>]>,
		id: AnyValue<Id>,
		organization: AnyValue<Organization<'name>>,
		person: AnyValue<Person<'contact_info, 'email, 'name, 'phone>>,
		store: Store<'pass, 'path, 'user>,
		title: AnyValue<&'title str>,
	) -> Result<&'arr [Self], Box<dyn Error>>;
}
