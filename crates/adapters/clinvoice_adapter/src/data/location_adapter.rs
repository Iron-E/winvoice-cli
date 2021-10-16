#![allow(clippy::wrong_self_convention)]

use clinvoice_data::{views::LocationView, Location};
use clinvoice_query as query;
use sqlx::{Acquire, Executor, Result};

use super::{Deletable, Updatable};

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
	///
	/// TODO: provide impl after https://github.com/rust-lang/rust/issues/60658
	async fn retrieve_view(
		connection: impl 'async_trait + Acquire<'_, Database = <Self as Deletable>::Db> + Send,
		query: &query::Location,
	) -> Result<Vec<LocationView>>;
	// where
	// 	for<'c> &'c mut <<Self as Deletable>::Db as Database>::Connection: Executor<'c, Database = <Self as Deletable>::Db>,
	// 	for<'c> &'c mut Transaction<'c, <Self as Deletable>::Db>: Executor<'c, Database = <Self as Deletable>::Db>,
	// {
	// 	let mut transaction = connection.begin().await?;
	// 	let inners = Self::retrieve(&mut transaction, query).await?;
	// 	todo!()
	// }
}
