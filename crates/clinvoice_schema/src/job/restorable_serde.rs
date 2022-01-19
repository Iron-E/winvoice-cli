use super::Job;
use crate::RestorableSerde;

impl RestorableSerde for Job
{
	fn restore(&mut self, original: &Self)
	{
		self.client.restore(&original.client);
		self.id = original.id;
	}
}
