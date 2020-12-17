use chrono;

pub struct Timesheet<'work_note, 'work_notes, Tz> where Tz : chrono::TimeZone
{
	pub time_begin: chrono::DateTime<Tz>,
	pub time_end: chrono::DateTime<Tz>,

	pub work_notes: &'work_notes[&'work_note str],
}
