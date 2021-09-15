#![allow(clippy::wrong_self_convention)]

use std::error::Error;

use clinvoice_data::{Person, views::PersonView};
use clinvoice_query as query;

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait PersonAdapter:
	Deletable<Error = <Self as PersonAdapter>::Error>
	+ Updatable<Error = <Self as PersonAdapter>::Error>
{
	type Error: From<super::Error> + Error;

	/// # Summary
	///
	/// Create a new [`Person`] on the database.
	///
	/// # Parameters
	///
	/// See [`Person`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	async fn create(name: String, pool: Self::Pool) -> Result<Person, <Self as PersonAdapter>::Error>;

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
	) -> Result<Vec<Person>, <Self as PersonAdapter>::Error>;

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
	) -> Result<Vec<PersonView>, <Self as PersonAdapter>::Error>;
}
