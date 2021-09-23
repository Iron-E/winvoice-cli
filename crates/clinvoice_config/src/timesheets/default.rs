use super::{Duration, Timesheets};

impl Default for Timesheets
{
	fn default() -> Self
	{
		Self {
			default_increment: Duration::from_secs(300),
		}
	}
}
