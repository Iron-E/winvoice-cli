use super::Job;
use crate::{RestorableSerde, RestoreResult};

impl RestorableSerde for Job
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.id = original.id;
		self.client.try_restore(&original.client)
	}
}
