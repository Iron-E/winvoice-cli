use chrono;

use super::{invoice::Invoice, job::Job, timesheet::Timesheet};

pub struct CLInvoice<'job_client, 'job_objective, 'job_objectives, 'job_note, 'job_notes, 'timesheets, 'timesheet_note, 'timesheet_notes, Tz>
	where Tz : chrono::TimeZone
{
	pub invoice: Invoice<Tz>,
	pub job: Job<'job_client, 'job_objective, 'job_objectives, 'job_note, 'job_notes, Tz>,
	pub timesheet: &'timesheets[Timesheet<'timesheet_note, 'timesheet_notes, Tz>],
}
