use crate::{Invoice, Timesheet, Id};
use std::collections::BTreeSet;
use chrono::{DateTime, Utc};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A [`Job`] contains all of the information which pertains to the specific
/// reasons that a [client](Organization) has contacted the user's
/// [employer](Organization) / the user.
///
/// It also defines the scope of the problem which is to be solved before an
/// [`Invoice`] is issued.
///
/// # Remarks
///
/// The [`Job`] can be thought of similarly to a support ticket. Whereas other
/// structures may define [the method of payment](Invoice),
/// [client](Organization) information, and [work periods](Timesheet)— this
/// structure defines what work _may_ performed.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Job<'currency, 'objectives, 'notes, 'work_notes>
{
	/// # Summary
	///
	/// The [`Organization`](crate::Organization) who the work is being performed for.
	pub client_id: Id,

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
	/// The __unique__ number of the [`Job`].
	///
	/// # Remarks
	///
	/// Should be automatically generated.
	pub id: Id,

	/// # Summary
	///
	/// The [`Invoice`] which will be sent to the [client](Organization) after the [`Job`] is done.
	pub invoice: Invoice<'currency>,

	/// # Summary
	///
	/// Important things to know about the work that has been performed.
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Images on the website now point to the correct location.
	/// * The PDF application has been replaced with a Google Form.
	/// * Customer support has been contacted and will reach out to you within X days.
	/// ```
	pub notes: &'notes str,

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
	pub objectives: &'objectives str,

	/// # Summary
	///
	/// The periods of time during which work was performed for this [`Job`].
	pub timesheets: BTreeSet<Timesheet<'work_notes>>,
}
