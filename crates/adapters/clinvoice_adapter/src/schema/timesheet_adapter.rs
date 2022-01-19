use clinvoice_match::MatchTimesheet;
use clinvoice_schema::{Employee, Job, Timesheet};
use sqlx::{Pool, Result};

use crate::{Deletable, Updatable};

#[async_trait::async_trait]
pub trait TimesheetAdapter:
	Deletable<Entity = Timesheet>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// # Summary
	///
	/// Create a new [`Timesheet`] on the database.
	///
	/// # Parameters
	///
	/// See [`Timesheet`].
	///
	/// # Returns
	///
	/// The newly created [`Timesheet`].
	async fn create(
		connection: &Pool<<Self as Deletable>::Db>,
		employee: Employee,
		job: Job,
	) -> Result<<Self as Deletable>::Entity>;

	/// # Summary
	///
	/// Retrieve some [`Timesheet`]s from the database using a [query](MatchTimesheet).
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Timesheet`]s.
	async fn retrieve(
		connection: &Pool<<Self as Deletable>::Db>,
		match_condition: MatchTimesheet,
	) -> Result<Vec<<Self as Deletable>::Entity>>;
}
