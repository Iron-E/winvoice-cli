//! Misc utilities for CLInvoice.

use core::fmt::Display;

use clinvoice_schema::chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike, Utc};

use crate::fmt;

/// Create a [`DateTime<Utc>`] out of some [`Local`] [`NaiveDateTime`].
pub(crate) fn naive_local_datetime_to_utc(d: NaiveDateTime) -> DateTime<Utc>
{
	Local
		.ymd(d.year(), d.month(), d.day())
		.and_hms(d.hour(), d.minute(), d.second())
		.into()
}

/// Indicate with [`println!`] that a value of type `TCreated` — identified by `id` — has been
/// created successfully.
pub(super) fn report_action<TCreated, TId>(action: &str, id: TId)
where
	TId: Display,
{
	println!("{} {id} has been {action}.", fmt::type_name::<TCreated>());
}
