use clinvoice_adapter::data::PersonAdapter;
use clinvoice_data::{views::PersonView, Person};
use clinvoice_query as query;
use sqlx::{Executor, Postgres, Result};

use super::PostgresPerson;

#[async_trait::async_trait]
impl PersonAdapter for PostgresPerson
{
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
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		name: String,
	) -> Result<Person>
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
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Person,
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
		connection: impl 'async_trait + Executor<'_, Database = Postgres>,
		query: &query::Person,
	) -> Result<Vec<PersonView>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test]
	async fn create() {}

	#[tokio::test]
	async fn retrieve()
	{
		// TODO: write test + `retrieve_view`
	}
}
