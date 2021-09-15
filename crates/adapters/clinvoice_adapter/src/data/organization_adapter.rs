#![allow(clippy::wrong_self_convention)]

use std::error::Error;

use clinvoice_data::{views::OrganizationView, Location, Organization};
use clinvoice_query as query;

use super::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait OrganizationAdapter:
	Deletable<Error = <Self as OrganizationAdapter>::Error>
	+ Updatable<Error = <Self as OrganizationAdapter>::Error>
{
	type Error: From<super::Error> + Error;

	/// # Summary
	///
	/// Create a new [`Organization`] on the database.
	///
	/// # Parameters
	///
	/// See [`Organization`].
	///
	/// # Returns
	///
	/// The newly created [`Organization`].
	async fn create(
		location: Location,
		name: String,
		pool: Self::Pool,
	) -> Result<Organization, <Self as OrganizationAdapter>::Error>;

	/// # Summary
	///
	/// Retrieve some [`Organization`]s from the database using a [query](query::Organization).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Organization`]s.
	async fn retrieve(
		query: &query::Organization,
		pool: Self::Pool,
	) -> Result<Vec<Organization>, <Self as OrganizationAdapter>::Error>;

	/// # Summary
	///
	/// Retrieve some [`OrganizationView`]s from the database using a [query](query::Organization).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`OrganizationView`]s.
	async fn retrieve_view(
		query: &query::Organization,
		pool: Self::Pool,
	) -> Result<Vec<OrganizationView>, <Self as OrganizationAdapter>::Error>;
}
