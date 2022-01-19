use super::Location;
use crate::RestorableSerde;

impl RestorableSerde for Location
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
	}
}
