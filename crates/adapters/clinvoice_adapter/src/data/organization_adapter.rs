use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::{Employee, Location, Organization, Id, views::OrganizationView},
	std::error::Error,
};

pub trait OrganizationAdapter<'pass, 'path, 'user, E>  :
	Deletable<E> +
	Initializable<E> +
	Into<Organization> +
	Into<Result<Vec<Employee>, E>> +
	Into<Result<Location, E>> +
	Into<Result<OrganizationView, E>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable<E> +
where
	 E : Error,
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
		location: Location,
		name: &str,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, E>;

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
	) -> Result<Vec<Self>, E>;
}
