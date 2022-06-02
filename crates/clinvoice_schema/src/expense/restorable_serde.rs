use super::Expense;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Expense
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		self.timesheet_id = original.timesheet_id;
		Ok(())
	}
}
