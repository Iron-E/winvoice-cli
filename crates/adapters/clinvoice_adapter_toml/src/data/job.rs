use clinvoice_data::{chrono::TimeZone, Job};

/// # Summary
///
/// A wrapper around [`Job`] for use with TomlDB.
pub struct TomlJob<'objectives,  'names, 'notes, 'rep_title, 'timesheets, 'timesheet_note, TZone>
(
	Job<'objectives,  'names, 'notes, 'rep_title, 'timesheets, 'timesheet_note, TZone>,
) where
	'timesheet_note : 'timesheets,
	TZone           : 'timesheets + TimeZone,
;
