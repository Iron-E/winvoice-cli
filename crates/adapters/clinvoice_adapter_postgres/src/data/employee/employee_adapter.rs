use
{
	std::collections::HashMap,

	super::PostgresEmployee,
	crate::data::{Error, Result},

	clinvoice_adapter::
	{
		data::{EmployeeAdapter, Error as DataError, Initializable, Updatable},
		Store,
	},
	clinvoice_data::{Contact, Employee, EmployeeStatus, Organization, Person},
	clinvoice_query as query,
};

impl EmployeeAdapter for PostgresEmployee<'_, '_>
{
	type Error = Error;

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
		contact_info: HashMap<String, Contact>,
		organization: Organization,
		person: Person,
		status: EmployeeStatus,
		title: String,
		store: &Store,
	) -> Result<Employee>
	{
		todo!()
	}

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
	fn retrieve(query: &query::Employee, store: &Store) -> Result<Vec<Employee>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[test]
	fn create()
	{
	}

	#[test]
	fn retrieve()
	{
	}
}
