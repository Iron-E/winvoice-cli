use super::Organization;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Organization
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		self.location.try_restore(&original.location)
	}
}
