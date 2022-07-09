use core::time::Duration;

use clinvoice_match::MatchJob;
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Invoice,
	Job,
	Organization,
};
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Job`]s.
#[async_trait::async_trait]
pub trait JobAdapter:
	Deletable<Entity = Job>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return a new [`Job`] via the `connection`.
	#[allow(clippy::too_many_arguments)]
	async fn create(
		connection: &Pool<<Self as Deletable>::Db>,
		client: Organization,
		date_close: Option<DateTime<Utc>>,
		date_open: DateTime<Utc>,
		increment: Duration,
		invoice: Invoice,
		notes: String,
		objectives: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// Retrieve all [`Job`]s (via `connection`) that match the `match_condition`.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchJob,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
