use
{
	super::{MatchWhen, Deletable, Initializable, Updatable},
	crate::{DynamicResult, Store},
	clinvoice_data::{Contact, Employee, EmployeeStatus, Id, Organization, Person, views::EmployeeView},
	std::collections::HashSet,
};

pub trait EmployeeAdapter<'pass, 'path, 'user> :
	Deletable +
	Initializable<'pass, 'path, 'user> +
	Into<Employee> +
	Into<DynamicResult<EmployeeView>> +
	Into<DynamicResult<Organization>> +
	Into<DynamicResult<Person>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
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
	fn create<'title>(
		contact_info: HashSet<Contact>,
		organization: Organization,
		person: Person,
		title: &'title str,
		status: EmployeeStatus,
		store: Store<'pass, 'path, 'user>,
	) -> DynamicResult<Self>;

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
	) -> DynamicResult<HashSet<Self>>;
}
