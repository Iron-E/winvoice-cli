mod display;
mod exchangeable;

use clinvoice_finance::Money;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::InvoiceDate;

/// Information about payment for the completion of a [`Job`](super::Job).
///
/// # Examples
///
/// ```rust
/// use clinvoice_schema::{chrono::Utc, Currency, Invoice, InvoiceDate, Money};
///
/// let _ = Invoice {
///   date: Some(InvoiceDate {
///     issued: Utc::now(),
///     paid: None,
///   }),
///   hourly_rate: Money::new(50_00, 2, Currency::Usd),
/// };
/// ```
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Invoice
{
	/// The date which the [`Invoice`] was sent to and/or paid by the [`Job`](super::Job)'s `client`, or
	/// [`None`] if the [`Invoice`] has not been sent yet.
	pub date: Option<InvoiceDate>,

	/// The amount of money to be charged for one hour of work. If the amount charged is on a
	/// per-[`Job`][job] basis (rather than hourly) set this to [`Money::default`].
	///
	/// This rate should be subdivided per each `increment` specified in the [`Job`][job] (e.g., if
	/// `interval` is 15 minutes and `hourly_rate` is $20, then the rate is _actually_ $5 per 15
	/// minute interval since the `time_start` of a [`Timesheet`](super::Timesheet)).
	///
	/// [job]: super::Job
	pub hourly_rate: Money,
}
