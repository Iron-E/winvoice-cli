use super::Location;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Location
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;

		if let Some(ref mut outer) = self.outer
		{
			// TODO: if-let chains
			if let Some(ref original_outer) = original.outer
			{
				outer.try_restore(original_outer)?;
			}
		}

		Ok(())
	}
}
