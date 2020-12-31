use clinvoice_data::{chrono::TimeZone, Invoice};

/// # Summary
///
/// A wrapper around [`Invoice`] for use with TomlDB.
pub struct TomlInvoice<Tz : TimeZone>
(
	Invoice<Tz>,
);
