//! Misc utilities for CLInvoice.

use clinvoice_schema::chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike, Utc};

/// Create a [`DateTime<Utc>`] out of some [`Local`] [`NaiveDateTime`].
pub(crate) fn naive_local_datetime_to_utc(d: NaiveDateTime) -> DateTime<Utc>
{
	Local
		.ymd(d.year(), d.month(), d.day())
		.and_hms(d.hour(), d.minute(), d.second())
		.into()
}
