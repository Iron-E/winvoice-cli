#![allow(clippy::wrong_self_convention)]

use std::error::Error;

use clinvoice_data::{Person, views::PersonView};
use clinvoice_query as query;
use sqlx::Executor;

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait PersonAdapter:
	Deletable<Entity = Person>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Error = <Self as Deletable>::Error>
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
	async fn create(
		connection: impl Executor<'_, Database = <Self as Deletable>::Db>,
		name: String,
	) -> Result<<Self as Deletable>::Entity, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	async fn retrieve(
		connection: impl Executor<'_, Database = <Self as Deletable>::Db>,
		query: &query::Person,
	) -> Result<Vec<<Self as Deletable>::Entity>, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Retrieve some [`PersonView`]s from the database using a [query](query::Person).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`PersonView`]s.
	async fn retrieve_view(
		connection: impl Executor<'_, Database = <Self as Deletable>::Db>,
		query: &query::Person,
	) -> Result<Vec<PersonView>, <Self as Deletable>::Error>;
}
