use super::Employee;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Employee
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		Ok(())
	}
}
