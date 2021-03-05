use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::{Employee, Location, Organization, Id, views::OrganizationView},
	std::error::Error,
};

pub trait OrganizationAdapter<'store>  :
	Deletable<Error=<Self as OrganizationAdapter<'store>>::Error> +
	Initializable<Error=<Self as OrganizationAdapter<'store>>::Error> +
	Into<Organization> +
	Into<Result<Vec<Employee>, <Self as OrganizationAdapter<'store>>::Error>> +
	Into<Result<Location, <Self as OrganizationAdapter<'store>>::Error>> +
	Into<Result<OrganizationView, <Self as OrganizationAdapter<'store>>::Error>> +
	Updatable<Error=<Self as OrganizationAdapter<'store>>::Error> +
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
	fn create(location: Location, name: &str, store: &'store Store) -> Result<Organization, <Self as OrganizationAdapter<'store>>::Error>;

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
		store: &Store,
	) -> Result<Vec<Organization>, <Self as OrganizationAdapter<'store>>::Error>;
}
