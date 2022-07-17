//! Misc utilities for CLInvoice.

use clinvoice_schema::chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike};

/// Create a [`DateTime<Local>`] out of some [`NaiveDateTime`].
pub(crate) fn naive_datetime_to_local(d: NaiveDateTime) -> DateTime<Local>
{
	Local
		.ymd(d.year(), d.month(), d.day())
		.and_hms(d.hour(), d.minute(), d.second())
}
