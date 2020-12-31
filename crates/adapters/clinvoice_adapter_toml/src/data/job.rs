use clinvoice_data::{chrono::TimeZone, Job};

/// # Summary
///
/// A wrapper around [`Job`] for use with TomlDB.
pub struct TomlJob<'objectives, 'notes, 'timesheets, 'timesheet_note, Tz : TimeZone>
(
	Job<'objectives, 'notes, 'timesheets, 'timesheet_note, Tz>,
);
