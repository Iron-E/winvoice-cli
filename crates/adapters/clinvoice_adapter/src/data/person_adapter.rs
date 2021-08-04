#![allow(clippy::wrong_self_convention)]

use std::error::Error;

use clinvoice_data::Person;
use clinvoice_query as query;

use super::{
	Deletable,
	Initializable,
	Updatable,
};
use crate::Store;

pub trait PersonAdapter:
	Deletable<Error = <Self as PersonAdapter>::Error>
	+ Initializable<Error = <Self as PersonAdapter>::Error>
	+ Updatable<Error = <Self as PersonAdapter>::Error>
{
	type Error: From<super::Error> + Error;

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
	fn create(name: String, store: &Store) -> Result<Person, <Self as PersonAdapter>::Error>;

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
	fn retrieve(
		query: &query::Person,
		store: &Store,
	) -> Result<Vec<Person>, <Self as PersonAdapter>::Error>;
}
