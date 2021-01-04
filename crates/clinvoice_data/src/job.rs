use crate::{Invoice, Organization, Timesheet};

use chrono::{DateTime, TimeZone};

/// # Summary
///
/// A [`Job`] contains all of the information which pertains to the specific
/// reasons that a [`Client`] has contacted the user's
/// [employer](crate::Organization) / the user.
///
/// It also defines the scope of the problem which is to be solved before an
/// [`Invoice`][invoice] is issued.
///
/// # Remarks
///
/// The [`Job`] can be thought of similarly to a support ticket. Whereas other
/// structures may define [the method of payment][invoice], [`Client`]
/// information, and [work periods](Timesheet)— this structure defines what
/// work _may_ performed.
///
/// [invoice]: super::invoice::Invoice
pub struct Job<'objectives,  'names, 'notes, 'rep_title, 'timesheets, 'timesheet_note, TZone> where
	'timesheet_note : 'timesheets,
	TZone           : 'timesheets + TimeZone,
{
	/// # Summary
	///
	/// The date upon which the client accepted the work as "complete".
	pub date_close: Option<DateTime<TZone>>,

	/// # Summary
	///
	/// The date upon which the client requested the work.
	pub date_open: DateTime<TZone>,

	/// # Summary
	///
	/// The client who the work is being performed for.
	pub client: Organization<'names, 'rep_title>,

	/// # Summary
	///
	/// The __unique__ number of the [`Job`].
	///
	/// # Remarks
	///
	/// Should be automatically generated.
	pub id: u64,

	/// # Summary
	///
	/// The [`Invoice`] which will be sent to the [`Client`] after the [`Job`] is done.
	pub invoice: Invoice<TZone>,

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
	pub timesheets: &'timesheets [Timesheet<'timesheet_note, TZone>]
}
