use super::Employee;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Employee
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.contact_info.try_restore(&original.contact_info)?;
		self.id = original.id;
		self.organization.try_restore(&original.organization)?;
		self.person.try_restore(&original.person)
	}
}
