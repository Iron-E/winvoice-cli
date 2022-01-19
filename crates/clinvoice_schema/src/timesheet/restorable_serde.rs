use super::Timesheet;
use crate::RestorableSerde;

impl RestorableSerde for Timesheet
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
		self.employee.restore(&original.employee);
		self.job.restore(&original.job);
	}
}
