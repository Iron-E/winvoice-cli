use chrono;

pub struct Job<'client, 'objective, 'objectives, 'note, 'notes, Tz>
	where Tz : chrono::TimeZone
{
	pub id: u64,
	pub client_name: &'client str,

	pub open_date: chrono::DateTime<Tz>,
	pub close_date: chrono::DateTime<Tz>,

	pub objectives: &'objectives[&'objective str],
	pub notes: &'notes[&'note str],
}
