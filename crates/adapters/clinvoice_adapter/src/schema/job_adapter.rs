use core::time::Duration;

use clinvoice_match::{MatchJob, MatchRow};
use clinvoice_schema::{
	chrono::{DateTime, Utc},
	Invoice,
	Job,
	Organization,
};
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

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
		connection: &Pool<<Self as Deletable>::Db>,
		client: Organization,
		date_close: Option<DateTime<Utc>>,
		date_open: DateTime<Utc>,
		increment: Duration,
		invoice: Invoice,
		notes: String,
		objectives: String,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`Job`]s from the database using a [query](MatchJob).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: &MatchRow<MatchJob>,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
