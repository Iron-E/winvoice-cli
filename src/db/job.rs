use chrono::{DateTime, TimeZone};
use super::{client::Client, employee::Employee, invoice::Invoice, timesheet::Timesheet};

/// # Summary
///
/// A [`Job`] contains all of the information which pertains to the specific
/// reasons that a [`Client`] has contacted the user's
/// [`Employer`](super::employer::Employer) / the user.
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
pub struct Job<'objectives,  'notes, 'timesheets,  'timesheet_note, Tz> where Tz : TimeZone
{
	/// # Summary
	///
	/// The date upon which the client accepted the work as "complete".
	pub date_close: Option<DateTime<Tz>>,

	/// # Summary
	///
	/// The date upon which the client requested the work.
	pub date_open: DateTime<Tz>,

	/// # Summary
	///
	/// The client who the work is being performed for.
	pub client: Client,

	/// # Summary
	///
	/// The employer who the work is being performed for.
	pub employer: Employee,

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
	pub invoice: Invoice<Tz>,

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
	pub timesheets: &'timesheets [Timesheet<'timesheet_note, Tz>]
}
