use clinvoice_data::{chrono::TimeZone, Invoice};

/// # Summary
///
/// A wrapper around [`Invoice`] for use with MongoDB.
pub struct MongoInvoice<Tz : TimeZone>
(
	Invoice<Tz>,
);
