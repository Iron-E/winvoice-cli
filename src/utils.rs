//! Misc utilities for CLInvoice.

mod identifiable;

use clinvoice_schema::chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike, Utc};
pub use identifiable::Identifiable;

use crate::fmt;

/// Create a [`DateTime<Utc>`] out of some [`Local`] [`NaiveDateTime`].
pub(crate) fn naive_local_datetime_to_utc(d: NaiveDateTime) -> DateTime<Utc>
{
	Local
		.ymd(d.year(), d.month(), d.day())
		.and_hms(d.hour(), d.minute(), d.second())
		.into()
}

/// Indicate with [`println!`] that a value of type `TActioned` — identified by `id` — has been
/// `action`ed.
pub(super) fn report_action<TActioned>(action: &str, actioned: &TActioned)
where
	TActioned: Identifiable,
{
	println!(
		"{} {} has been {action}.",
		fmt::type_name::<TActioned>(),
		actioned.id(),
	);
}
