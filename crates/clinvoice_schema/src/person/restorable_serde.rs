use super::Person;
use crate::RestorableSerde;

impl RestorableSerde for Person
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
	}
}
