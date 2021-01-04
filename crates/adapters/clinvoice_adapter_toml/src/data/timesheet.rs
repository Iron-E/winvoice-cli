use clinvoice_data::{chrono::TimeZone, Timesheet};

/// # Summary
///
/// A wrapper around [`Timesheet`] for use with TomlDB.
pub struct TomlTimesheet<'work_notes, TZone>
(
	Timesheet<'work_notes, TZone>,
) where
	TZone : TimeZone
;
