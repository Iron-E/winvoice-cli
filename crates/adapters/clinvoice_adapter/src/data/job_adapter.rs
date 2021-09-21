#![allow(clippy::wrong_self_convention)]

use clinvoice_data::{
	chrono::{DateTime, Utc},
	finance::Money,
	views::JobView,
	Job,
	Organization,
};
use clinvoice_query as query;
use sqlx::Executor;

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait JobAdapter:
	Deletable<Entity = Job>
	+ Updatable<
		Db = <Self as Deletable>::Db,
		Entity = <Self as Deletable>::Entity,
		Error = <Self as Deletable>::Error,
	>
{
	/// # Summary
	///
	/// Create a new [`Job`] on the database.
	///
	/// # Paramters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// The newly created [`Job`].
	async fn create(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: String,
	) -> Result<<Self as Deletable>::Entity, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Retrieve some [`Job`]s from the database using a [query](query::Job).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		query: &query::Job,
	) -> Result<Vec<<Self as Deletable>::Entity>, <Self as Deletable>::Error>;

	/// # Summary
	///
	/// Retrieve some [`JobView`]s from the database using a [query](query::Job).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`JobView`]s.
	async fn retrieve_view(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		query: &query::Job,
	) -> Result<Vec<JobView>, <Self as Deletable>::Error>;
}
