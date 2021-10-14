#![allow(clippy::wrong_self_convention)]

use core::time::Duration;

use clinvoice_data::{
	chrono::{DateTime, Utc},
	views::JobView,
	Job,
	Money,
	Organization,
};
use clinvoice_query as query;
use futures::Stream;
use sqlx::{Executor, Result};

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait JobAdapter:
	Deletable<Entity = Job>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
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
		increment: Duration,
		objectives: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`Job`]s from the database using a [query](query::Job).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve<'a, S>(
		connection: impl Executor<'a, Database = <Self as Deletable>::Db>,
		query: &query::Job,
	) -> S
	where
		S: Stream<Item = Result<<Self as Deletable>::Entity>>;

	/// # Summary
	///
	/// Retrieve some [`JobView`]s from the database using a [query](query::Job).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`JobView`]s.
	fn retrieve_view<'a, S>(
		connection: impl Executor<'a, Database = <Self as Deletable>::Db>,
		query: &query::Job,
	) -> S
	where
		S: Stream<Item = Result<JobView>>;
}
