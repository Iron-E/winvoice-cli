use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::Store,
	clinvoice_data::{Contact, Employee, EmployeeStatus, Id, Organization, Person, views::EmployeeView},
	std::error::Error,
};

pub trait EmployeeAdapter<'pass, 'path, 'user, E> :
	Deletable<E> +
	Initializable<E> +
	Into<Employee> +
	Into<Result<EmployeeView, E>> +
	Into<Result<Organization, E>> +
	Into<Result<Person, E>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable<E> +
where
	 E : Error,
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
		contact_info: Vec<Contact>,
		organization: Organization,
		person: Person,
		title: &str,
		status: EmployeeStatus,
		store: Store<'pass, 'path, 'user>,
	) -> Result<Self, E>;

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
		store: Store<'pass, 'path, 'user>,
	) -> Result<Vec<Self>, E>;
}
