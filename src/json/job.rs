use chrono;

pub struct Job<'client_name, 'job_note, 'job_notes, Tz> where Tz : chrono::TimeZone
{
	pub id: u64,
	pub client_name: &'client_name str,

	pub open_date: chrono::DateTime<Tz>,
	pub close_date: chrono::DateTime<Tz>,

	pub job_notes: &'job_notes[&'job_note str],
}
