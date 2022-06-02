use super::Organization;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Organization
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		self.contact_info.try_restore(&original.contact_info)?;
		self.location.try_restore(&original.location)
	}
}
