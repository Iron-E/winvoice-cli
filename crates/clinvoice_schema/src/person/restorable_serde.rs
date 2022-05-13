use super::Person;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Person
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		Ok(())
	}
}
