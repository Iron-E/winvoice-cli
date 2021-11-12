use clinvoice_query as query;
use clinvoice_schema::{views::OrganizationView, Location, Organization};
use sqlx::{Executor, Result};

use crate::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait OrganizationAdapter:
	Deletable<Entity = Organization>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
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
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		location: &Location,
		name: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`OrganizationView`]s from the database using a [query](query::Organization).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`OrganizationView`]s.
	async fn retrieve_view(
		connection: impl 'async_trait + Executor<'_, Database = <Self as Deletable>::Db>,
		match_condition: &query::Organization,
	) -> Result<Vec<OrganizationView>>;
}
