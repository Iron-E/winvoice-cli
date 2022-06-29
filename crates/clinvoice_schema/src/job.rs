mod default;
mod display;
mod exchangeable;
mod restorable_serde;

use core::time::Duration;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Id, Invoice, Organization};

/// # Summary
///
/// A view of [`Job`](crate::Job).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Job
{
	/// # Summary
	///
	/// The [`Organization`](crate::Organization) who the work is being performed for.
	pub client: Organization,

	/// # Summary
	///
	/// The date upon which the client accepted the work as "complete".
	pub date_close: Option<DateTime<Utc>>,

	/// # Summary
	///
	/// The [date](DateTime) upon which the client requested the work.
	pub date_open: DateTime<Utc>,

	/// # Summary
	///
	/// The [`Job`] number.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The amount of time between increments to the [`time_end`] on a [`Timesheet`].
	///
	/// # Example
	///
	/// * If `increment` is 15m…
	///   * A work begin time of 12:14 is set to 12:15.
	///   * A work end time of 13:29 is set to 13:30.
	/// * If `increment` is 5m…
	///   * A work begin time of 12:07 is set to 12:05.
	///   * A work end time of 13:31 is set to 13:30.
	/// * If `increment` is 0m…
	///   * A work begin time of 12:14 is not changed.
	///   * A work end time of 13:29 is not changed.
	///
	/// __Note__ that the duration does not have to be in even minutes. It can be any combination of
	/// days, hours, minutes, etc.
	#[cfg_attr(feature = "serde_support", serde(with = "humantime_serde"))]
	pub increment: Duration,

	/// # Summary
	///
	/// The [`Invoice`] which will be sent to the [client](Organization) after the [`Job`] is done.
	pub invoice: Invoice,

	/// # Summary
	///
	/// Important things to know about the work that has been performed.
	///
	/// # Example
	///
	/// __Note:__ this field may contain any valid markdown.
	///
	/// ```markdown
	/// * Images on the website now point to the correct location.
	/// * The PDF application has been replaced with a Google Form.
	/// * Customer support has been contacted and will reach out to you within X days.
	/// ```
	pub notes: String,

	/// # Summary
	///
	/// What problems will be addressed before the [`Job`] is closed.
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Fix website rendering issue.
	/// * Replace PDF with Google Form.
	/// * Contact customer support for X hardware device.
	/// ```
	pub objectives: String,
}
