use
{
	super::{Deletable, Initializable, MatchWhen, Updatable},
	crate::Store,
	clinvoice_data::{Employee, Location, Organization, Id, views::OrganizationView},
	std::error::Error,
};

pub trait OrganizationAdapter  :
	Deletable<Error=<Self as OrganizationAdapter>::Error> +
	Initializable<Error=<Self as OrganizationAdapter>::Error> +
	Into<Organization> +
	Into<Result<Vec<Employee>, <Self as OrganizationAdapter>::Error>> +
	Into<Result<Location, <Self as OrganizationAdapter>::Error>> +
	Into<Result<OrganizationView, <Self as OrganizationAdapter>::Error>> +
	Updatable<Error=<Self as OrganizationAdapter>::Error> +
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
		store: Store,
	) -> Result<Organization, <Self as OrganizationAdapter>::Error>;

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
		store: Store,
	) -> Result<Vec<Organization>, <Self as OrganizationAdapter>::Error>;
}
