use chrono;

pub struct Invoice <Tz> where Tz : chrono::TimeZone
{
	pub date_issued: chrono::DateTime<Tz>,
	pub date_paid: chrono::DateTime<Tz>,

	pub hourly_rate: f32,
}
