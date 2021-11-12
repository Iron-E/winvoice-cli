use clinvoice_query as query;
use clinvoice_schema::{views::LocationView, Location};
use sqlx::{Acquire, Executor, Result};

use crate::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait LocationAdapter:
	Deletable<Entity = Location>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
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
	) -> Result<<Self as Deletable>::Entity>;

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
	) -> Result<<Self as Deletable>::Entity>;

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
		match_condition: &query::Location,
	) -> Result<Vec<LocationView>>;
}
