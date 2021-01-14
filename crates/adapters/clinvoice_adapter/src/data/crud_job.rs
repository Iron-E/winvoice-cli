use super::{AnyValue, Deletable, Updatable};
use clinvoice_data::{chrono::{DateTime, TimeZone}, Id, Job, Organization, Timesheet};
use std::error::Error;

pub trait CrudJob<'err, 'objectives, 'name, 'notes, 'timesheets, 'title, 'work_notes, TZone> :
	Deletable<'err> +
	From<Job<'objectives, 'notes, 'timesheets, 'work_notes, TZone>> +
	Into<Organization<'name>> +
	Updatable<'err> +
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
		timesheets: &'timesheets [Timesheet<'work_notes, TZone>],
	) -> Result<Self, &'err dyn Error>;

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
		timesheets: AnyValue<&'timesheets [Timesheet<'work_notes, TZone>]>,
	) -> Result<Option<&'arr [Self]>, &'err dyn Error>;
}
