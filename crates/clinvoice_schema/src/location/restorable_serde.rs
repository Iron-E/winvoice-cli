use super::Location;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Location
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		Ok(())
	}
}
