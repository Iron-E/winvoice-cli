use super::{PATH, TomlJob};
use crate::util;
use clinvoice_adapter::{data::{AnyValue, JobAdapter}, Store};
use clinvoice_data::{chrono::{DateTime, TimeZone}, Id, Organization, Timesheet};
use std::error::Error;

impl<'objectives, 'name, 'notes, 'pass, 'path, 'timesheets, 'title, 'user, 'work_notes, TZone> JobAdapter<'objectives, 'name, 'notes, 'pass, 'path, 'timesheets, 'title, 'user, 'work_notes, TZone>
for TomlJob<'objectives, 'notes, 'timesheets, 'work_notes, 'pass, 'path, 'user, TZone>
where
	 'work_notes : 'timesheets,
	  TZone : 'timesheets + TimeZone,
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
		date_close: Option<DateTime<TZone>>,
		date_open: DateTime<TZone>,
		client: Organization<'name>,
		notes: &'notes str,
		store: Store<'pass, 'path, 'user>,
		timesheets: &'timesheets [Timesheet<'work_notes, TZone>],
	) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		return util::create_store_dir(store, PATH);
	}

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
		date_close: AnyValue<Option<DateTime<TZone>>>,
		date_open: AnyValue<DateTime<TZone>>,
		client_id: AnyValue<Organization<'name>>,
		id: AnyValue<Id>,
		notes: AnyValue<&'notes str>,
		store: Store<'pass, 'path, 'user>,
		timesheets: AnyValue<&'timesheets [Timesheet<'work_notes, TZone>]>,
	) -> Result<Option<&'arr [Self]>, Box<dyn Error>>
	{
		todo!()
	}
}

