use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::{Employee, Location, Organization, Id, views::OrganizationView},
	std::error::Error,
};

pub trait OrganizationAdapter<'pass, 'path, 'user>  :
	Deletable<Self::Error> +
	Initializable<Self::Error> +
	Into<Organization> +
	Into<Result<Vec<Employee>, Self::Error>> +
	Into<Result<Location, Self::Error>> +
	Into<Result<OrganizationView, Self::Error>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable<Self::Error> +
{
	type Error : Error;

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
	) -> Result<Self, Self::Error>;

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
	) -> Result<Vec<Self>, Self::Error>;
}
