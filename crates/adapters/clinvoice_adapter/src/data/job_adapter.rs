use super::{AnyValue, Deletable, Updatable};
use crate::Store;
use clinvoice_data::{chrono::{DateTime, Utc}, Id, Job, Organization, Timesheet};
use std::{collections::BTreeSet, error::Error};

pub trait JobAdapter<'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes> :
	Deletable<'pass, 'path, 'user> +
	Into<Job<'objectives, 'notes, 'work_notes>> +
	Into<Result<Organization<'name>, Box<dyn Error>>> +
	Into<Store<'pass, 'path, 'user>> +
	Updatable +
{
	/// # Summary
	///
	/// Create a new [`Person`] on the active [`Store`](crate::Store).
	///
	/// # Paramters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// The newly created [`Person`].
	fn create(
		date_close: Option<DateTime<Utc>>,
		date_open: DateTime<Utc>,
		client: Organization<'name>,
		notes: &'notes str,
		store: Store<'pass, 'path, 'user>,
		timesheets: BTreeSet<Timesheet<'work_notes>>,
	) -> Result<Self, Box<dyn Error>>;

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>;

	/// # Summary
	///
	/// Retrieve some [`Person`] from the active [`Store`](crate::Store).
	///
	/// # Parameters
	///
	/// See [`Job`].
	///
	/// # Returns
	///
	/// * An `Error`, if something goes wrong.
	/// * A list of matching [`Job`]s.
	fn retrieve<'arr>(
		date_close: AnyValue<Option<DateTime<Utc>>>,
		date_open: AnyValue<DateTime<Utc>>,
		client_id: AnyValue<Organization<'name>>,
		id: AnyValue<Id>,
		notes: AnyValue<&'notes str>,
		store: Store<'pass, 'path, 'user>,
		timesheets: AnyValue<BTreeSet<Timesheet<'work_notes>>>,
	) -> Result<Option<&'arr [Self]>, Box<dyn Error>>;
}
