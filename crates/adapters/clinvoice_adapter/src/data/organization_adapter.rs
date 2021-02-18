use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::{DynamicResult, Store},
	clinvoice_data::{Employee, Location, Organization, Id, views::OrganizationView},
	std::collections::HashSet,
};

pub trait OrganizationAdapter<'pass, 'path, 'user> :
	Deletable +
	Initializable<'pass, 'path, 'user> +
	Into<Organization> +
	Into<DynamicResult<HashSet<Employee>>> +
	Into<DynamicResult<Location>> +
	Into<DynamicResult<OrganizationView>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	fn create<'name>(
		location: Location,
		name: &'name str,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Self>;

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
	fn retrieve(
		id: MatchWhen<Id>,
		location: MatchWhen<Id>,
		name: MatchWhen<String>,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<HashSet<Self>>;
}
