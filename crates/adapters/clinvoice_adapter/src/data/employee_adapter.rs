use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::Store,
	clinvoice_data::{Contact, Employee, EmployeeStatus, Id, Organization, Person, views::EmployeeView},
	std::error::Error,
};

pub trait EmployeeAdapter :
	Deletable<Error=<Self as EmployeeAdapter>::Error> +
	Initializable<Error=<Self as EmployeeAdapter>::Error> +
	Into<Employee> +
	Into<Result<EmployeeView, <Self as EmployeeAdapter>::Error>> +
	Into<Result<Organization, <Self as EmployeeAdapter>::Error>> +
	Into<Result<Person, <Self as EmployeeAdapter>::Error>> +
	Updatable<Error=<Self as EmployeeAdapter>::Error> +
{
	type Error : Error;

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
		contact_info: Vec<Contact>,
		organization: Organization,
		person: Person,
		title: &str,
		status: EmployeeStatus,
		store: Store,
	) -> Result<Employee, <Self as EmployeeAdapter>::Error>;

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
		title: MatchWhen<String>,
		status: MatchWhen<EmployeeStatus>,
		store: Store,
	) -> Result<Vec<Employee>, <Self as EmployeeAdapter>::Error>;
}
