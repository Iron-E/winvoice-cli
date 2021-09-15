use clinvoice_data::views::PersonView;

use
{
	super::PostgresPerson,
	crate::data::{Error, Result},

	clinvoice_adapter::data::PersonAdapter,
	clinvoice_data::Person,
	clinvoice_query as query,
};

#[async_trait::async_trait]
impl PersonAdapter for PostgresPerson<'_>
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
	async fn create(name: String, pool: Self::Pool) -> Result<Person>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	async fn retrieve(
		query: &query::Person,
		pool: Self::Pool,
	) -> Result<Vec<Person>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	async fn retrieve_view(
		query: &query::Person,
		pool: Self::Pool,
	) -> Result<Vec<PersonView>>
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
