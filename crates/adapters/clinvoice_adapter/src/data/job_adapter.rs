#![allow(clippy::wrong_self_convention)]

use std::error::Error;

use clinvoice_data::{
	chrono::{DateTime, Utc},
	finance::Money,
	views::JobView,
	Job,
	Organization,
};
use clinvoice_query as query;

use super::{
	Deletable,
	Updatable,
};

#[async_trait::async_trait]
pub trait JobAdapter:
	Deletable<Error = <Self as JobAdapter>::Error>
	+ Updatable<Error = <Self as JobAdapter>::Error>
{
	type Error: From<super::Error> + Error;

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
		client: Organization,
		date_open: DateTime<Utc>,
		hourly_rate: Money,
		objectives: String,
		pool: Self::Pool,
	) -> Result<Job, <Self as JobAdapter>::Error>;

	/// # Summary
	///
	/// Retrieve some [`Job`]s from the database using a [query](query::Job).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(
		query: &query::Job,
		pool: Self::Pool,
	) -> Result<Vec<Job>, <Self as JobAdapter>::Error>;

	/// # Summary
	///
	/// Retrieve some [`JobView`]s from the database using a [query](query::Job).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`JobView`]s.
	async fn retrieve_view(
		query: &query::Job,
		pool: Self::Pool,
	) -> Result<Vec<JobView>, <Self as JobAdapter>::Error>;
}
