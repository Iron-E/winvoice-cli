use
{
	super::PostgresPerson,
	crate::data::{Error, Result},

	clinvoice_adapter::
	{
		data::{Error as DataError, Initializable, PersonAdapter, Updatable},
		Store,
	},
	clinvoice_data::Person,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl PersonAdapter for PostgresPerson<'_, '_>
{
	type Error = Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(name: String, store: &Store) -> Result<Person>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(query: &query::Person, store: &Store) -> Result<Vec<Person>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn create()
	{
	}

	#[tokio::test]
	async fn retrieve()
	{
	}
}
