use chrono::Utc;

use crate::Job;

impl Default for Job
{
	fn default() -> Self
	{
		Self {
			client: Default::default(),
			date_close: Default::default(),
			date_open: Utc::now(),
			id: Default::default(),
			increment: Default::default(),
			invoice: Default::default(),
			notes: Default::default(),
			objectives: Default::default(),
		}
	}
}
