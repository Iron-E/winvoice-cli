use super::{AnyValue, Deletable, Updatable};
use clinvoice_data::{Contact, Employee, Id, Organization, Person};
use std::error::Error;

pub trait CrudEmployee<'contact_info, 'email, 'err, 'name, 'phone, 'title> :
	Deletable<'err> +
	From<Employee<'contact_info, 'email, 'phone, 'title>> +
	Into<Organization<'name>> +
	Into<Person<'contact_info, 'email, 'name, 'phone>> +
	Updatable<'err> +
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
		title: &'title str,
	) -> Result<Self, &'err dyn Error>;

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
		title: AnyValue<&'title str>,
	) -> Result<&'arr [Self], &'err dyn Error>;
}
