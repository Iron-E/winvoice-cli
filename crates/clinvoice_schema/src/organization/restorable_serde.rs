use super::Organization;
use crate::RestorableSerde;

impl RestorableSerde for Organization
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
		self.location.restore(&original.location);
	}
}
