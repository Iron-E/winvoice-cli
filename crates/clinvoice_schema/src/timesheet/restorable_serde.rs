use super::Timesheet;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Timesheet
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		self.employee.try_restore(&original.employee)?;
		self.job.try_restore(&original.job)
	}
}
