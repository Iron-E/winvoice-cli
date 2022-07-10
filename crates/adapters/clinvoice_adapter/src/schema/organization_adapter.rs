use clinvoice_match::MatchOrganization;
use clinvoice_schema::{Location, Organization};
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Organization`]s.
#[async_trait::async_trait]
pub trait OrganizationAdapter:
	Deletable<Entity = Organization>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Organization`] via the `connection`.
	async fn create(
		connection: &Pool<<Self as Deletable>::Db>,
		location: Location,
		name: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// Retrieve all [`Organization`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchOrganization,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
