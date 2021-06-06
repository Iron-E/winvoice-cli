use
{
	super::PostgresOrganization,
	crate::data::{Error, Result},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, OrganizationAdapter, Updatable},
		Store,
	},
	clinvoice_data::{Location, Organization},
	clinvoice_query as query,
};

impl OrganizationAdapter for PostgresOrganization<'_, '_>
{
	type Error = Error;

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
	fn create(location: Location, name: String, store: &Store) -> Result<Organization>
	{
		todo!()
	}

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
	fn retrieve(query: &query::Organization, store: &Store) -> Result<Vec<Organization>>
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
