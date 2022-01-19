use chrono::Utc;

use crate::Timesheet;

impl Default for Timesheet
{
	fn default() -> Self
	{
		Timesheet {
			id: Default::default(),
			employee: Default::default(),
			expenses: Default::default(),
			job: Default::default(),
			time_begin: Utc::now(),
			time_end: Default::default(),
			work_notes: Default::default(),
		}
	}
}
