use chrono;

use super::{invoice::Invoice, job::Job, timesheet::Timesheet};

/// # Summary
///
/// A `CLInvoice` represents the entirety of the business logic which this application aims to
/// maintain.
pub struct CLInvoice<'job_client, 'job_objectives, 'job_notes, 'timesheets, 'timesheet_note, Tz>
	where Tz : chrono::TimeZone
{
	/// # Summary
	///
	/// The [`Invoice`] which will be sent after the [`Self::job`] is done.
	pub invoice: Invoice<Tz>,

	/// # Summary
	///
	/// The [`Job`] which the client has asked to be performed.
	pub job: Job<'job_client, 'job_objectives, 'job_notes, Tz>,

	/// # Summary
	///
	/// The [`Timesheet`]s which represent the work done on the [`Self::job`].
	pub timesheet: &'timesheets[Timesheet<'timesheet_note, Tz>],
}
