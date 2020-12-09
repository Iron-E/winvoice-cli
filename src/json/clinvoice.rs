use chrono;

use super::invoice::Invoice;
use super::job::Job;
use super::timesheet::Timesheet;

pub struct CLInvoice <'client_name, 'job_note, 'job_notes, 'timesheets, 'work_note, 'work_notes, Tz> where Tz : chrono::TimeZone
{
	pub invoice: Invoice<Tz>,
	pub job: Job<'client_name, 'job_note, 'job_notes, Tz>,
	pub timesheet: &'timesheets[Timesheet<'work_note, 'work_notes, Tz>],
}
