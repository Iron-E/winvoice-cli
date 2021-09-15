#![allow(clippy::wrong_self_convention)]

use std::{borrow::Cow::Owned, error::Error};

use clinvoice_data::{views::LocationView, Location};
use clinvoice_query as query;
use futures::FutureExt;

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait LocationAdapter:
	Deletable<Error = <Self as LocationAdapter>::Error>
	+ Updatable<Error = <Self as LocationAdapter>::Error>
{
	type Error: From<super::Error> + Error;

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
		name: String,
		pool: Self::Pool,
	) -> Result<Location, <Self as LocationAdapter>::Error>;

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
	async fn create_inner(&self, name: String)
		-> Result<Location, <Self as LocationAdapter>::Error>;

	/// # Summary
	///
	/// Get the [`Location`]s which contain this [`Location`].
	async fn outers(
		location: &Location,
		pool: Self::Pool,
	) -> Result<Vec<Location>, <Self as LocationAdapter>::Error>
	{
		let mut outer_locations = Vec::new();
		let mut outer_id = location.outer_id;

		while let Some(id) = outer_id
		{
			let query = query::Location {
				id: query::Match::EqualTo(Owned(id)),
				..Default::default()
			};

			Self::retrieve(&query, pool.clone())
				.map(|result| {
					result.and_then(|retrieved| {
						retrieved
							.into_iter()
							.next()
							.map(|adapted_location| {
								outer_id = adapted_location.outer_id;
								outer_locations.push(adapted_location);
							})
							.ok_or_else(|| super::Error::DataIntegrity(id).into())
					})
				})
				.await?;
		}

		Ok(outer_locations)
	}

	/// # Summary
	///
	/// Retrieve some [`LocationView`]s from the database using a [query](query::Location).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`LocationView`]s.
	async fn retrieve(
		query: &query::Location,
		pool: Self::Pool,
	) -> Result<Vec<Location>, <Self as LocationAdapter>::Error>;

	/// # Summary
	///
	/// Retrieve some [`LocationView`]s from the database using a [query](query::Location).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`LocationView`]s.
	async fn retrieve_view(
		query: &query::Location,
		pool: Self::Pool,
	) -> Result<Vec<LocationView>, <Self as LocationAdapter>::Error>;
}
