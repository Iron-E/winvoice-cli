use super::JobView;
use crate::views::RestorableSerde;

impl RestorableSerde for JobView
{
	fn restore(&mut self, original: &Self)
	{
		self.client.restore(&original.client);
		self.id = original.id;
	}
}
