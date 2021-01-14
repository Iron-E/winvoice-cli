use super::{AnyValue, Deletable, Updatable};
use clinvoice_data::{Employee, Id, Location, Organization};
use std::{collections::HashSet, error::Error};

pub trait CrudOrganization<'contact_info, 'email, 'err, 'name, 'phone, 'title> :
	Deletable<'err> +
	From<Organization<'name>> +
	Into<HashSet<Employee<'contact_info, 'email, 'phone, 'title>>> +
	Into<Location<'name>> +
	Updatable<'err> +
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
	) -> Result<Self, &'err dyn Error>;

	/// # Summary
	///
	/// Retrieve some [`Organization`] from the active [`Store`](crate::Store).
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
	) -> Result<Option<&'arr [Self]>, &'err dyn Error>;
}
