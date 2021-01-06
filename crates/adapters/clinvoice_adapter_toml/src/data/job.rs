use clinvoice_data::{chrono::TimeZone, Job};

/// # Summary
///
/// A wrapper around [`Job`] for use with TomlDB.
pub struct TomlJob<'objectives, 'notes, TZone>
(
	Job<'objectives, 'notes, TZone>,
) where
	TZone : TimeZone,
;
