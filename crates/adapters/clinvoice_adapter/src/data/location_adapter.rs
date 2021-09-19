#![allow(clippy::wrong_self_convention)]

use clinvoice_data::{views::LocationView, Location};
use clinvoice_query as query;
use sqlx::{Acquire, Executor};

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait LocationAdapter:
	Deletable<Entity = Location>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Error = <Self as Deletable>::Error>
{
	/// # Summary
	///
	/// Create a new [`Location`] on the database.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// The created [`Location`].
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		name: String,
	) -> Result<<Self as Deletable>::Entity, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Create a new [`Location`] on the database which is inside of `self`.
	///
	/// # Parameters
	///
	/// See [`Location`].
	///
	/// # Returns
	///
	/// The created [`Location`].
	async fn create_inner(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		outer: &<Self as Deletable>::Entity,
		name: String,
	) -> Result<<Self as Deletable>::Entity, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Retrieve some [`LocationView`]s from the database using a [query](query::Location).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`LocationView`]s.
	async fn retrieve(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		query: &query::Location,
	) -> Result<Vec<<Self as Deletable>::Entity>, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Get the [`Location`]s which contain this [`Location`].
	async fn retrieve_outers(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		location: &<Self as Deletable>::Entity,
	) -> Result<Vec<<Self as Deletable>::Entity>, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Retrieve some [`LocationView`]s from the database using a [query](query::Location).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`LocationView`]s.
	async fn retrieve_view(
		connection: impl 'async_trait + Acquire<'_, Database = <Self as Deletable>::Db> + Send,
		query: &query::Location,
	) -> Result<Vec<LocationView>, <Self as Deletable>::Error>;
}
