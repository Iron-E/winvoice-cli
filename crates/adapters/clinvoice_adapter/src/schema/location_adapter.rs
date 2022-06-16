use clinvoice_match::MatchLocation;
use clinvoice_schema::Location;
use sqlx::{Pool, Result};

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
		connection: &Pool<<Self as Deletable>::Db>,
		name: String,
		outer: Option<<Self as Deletable>::Entity>,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`Location`]s from the database using a [query](MatchLocation).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Location`]s.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchLocation,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
