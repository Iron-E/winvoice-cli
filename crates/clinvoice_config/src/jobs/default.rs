use super::{Duration, Jobs};

impl Default for Jobs
{
	fn default() -> Self
	{
		Self {
			default_increment: Duration::from_secs(300),
		}
	}
}
