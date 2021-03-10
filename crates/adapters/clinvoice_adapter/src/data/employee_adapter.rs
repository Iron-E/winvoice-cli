use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::Store,
	clinvoice_data::{Contact, Employee, EmployeeStatus, Id, Organization, Person, views::EmployeeView},
	std::error::Error,
};

pub trait EmployeeAdapter<'store> :
	Deletable<Error=<Self as EmployeeAdapter<'store>>::Error> +
	Initializable<Error=<Self as EmployeeAdapter<'store>>::Error> +
	Into<Employee> +
	Into<Result<EmployeeView, <Self as EmployeeAdapter<'store>>::Error>> +
	Into<Result<Organization, <Self as EmployeeAdapter<'store>>::Error>> +
	Into<Result<Person, <Self as EmployeeAdapter<'store>>::Error>> +
	Updatable<Error=<Self as EmployeeAdapter<'store>>::Error> +
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
		status: EmployeeStatus,
		title: &str,
		store: &'store Store,
	) -> Result<Employee, <Self as EmployeeAdapter<'store>>::Error>;

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
		store: &Store,
	) -> Result<Vec<Employee>, <Self as EmployeeAdapter<'store>>::Error>;
}
