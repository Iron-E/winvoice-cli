use super::TomlJob;
use crate::util;
use clinvoice_adapter::{data::{AnyValue, JobAdapter}, Store};
use clinvoice_data::{chrono::{DateTime, Utc}, Id, Organization, Timesheet};
use std::{collections::BTreeSet, error::Error};

impl<'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes> JobAdapter<'objectives, 'name, 'notes, 'pass, 'path, 'title, 'user, 'work_notes>
for TomlJob<'objectives, 'notes, 'work_notes, 'pass, 'path, 'user>
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
	) -> Result<Self, Box<dyn Error>>
	{
		todo!()
	}

	/// # Summary
	///
	/// Initialize the database for a given [`Store`].
	fn init(store: &Store<'pass, 'path, 'user>) -> Result<(), Box<dyn Error>>
	{
		util::create_store_dir(&TomlJob::path(store))?;
		return Ok(());
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
		date_close: AnyValue<Option<DateTime<Utc>>>,
		date_open: AnyValue<DateTime<Utc>>,
		client_id: AnyValue<Organization<'name>>,
		id: AnyValue<Id>,
		notes: AnyValue<&'notes str>,
		store: Store<'pass, 'path, 'user>,
		timesheets: AnyValue<BTreeSet<Timesheet<'work_notes>>>,
	) -> Result<Option<&'arr [Self]>, Box<dyn Error>>
	{
		todo!()
	}
}

#[cfg(test)]
mod tests
{
	use super::{JobAdapter, TomlJob, util};
	use std::{fs, io};

	#[test]
	fn test_init() -> Result<(), io::Error>
	{
		return util::test_temp_store(|store|
		{
			// Assert that the function can initialize the store.
			assert!(TomlJob::init(store).is_ok());

			// Create filepath for temporary test file.
			let filepath = TomlJob::path(store).join("testfile.txt");

			// Assert that creation of a file inside the initialized space is done
			assert!(fs::write(&filepath, "").is_ok());

			// Assert that the function will still return OK with files in the directory.
			assert!(TomlJob::init(store).is_ok());

			// Assert cleanup
			assert!(fs::remove_file(filepath).is_ok());
		});
	}
}
