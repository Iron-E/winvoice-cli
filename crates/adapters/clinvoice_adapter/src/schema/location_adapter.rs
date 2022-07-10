use clinvoice_match::MatchLocation;
use clinvoice_schema::Location;
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Location`]s.
#[async_trait::async_trait]
pub trait LocationAdapter:
	Deletable<Entity = Location>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Location`] via the `connection`.
	async fn create(
		connection: &Pool<<Self as Deletable>::Db>,
		name: String,
		outer: Option<<Self as Deletable>::Entity>,
	) -> Result<<Self as Deletable>::Entity>;

	/// Retrieve all [`Location`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchLocation,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
