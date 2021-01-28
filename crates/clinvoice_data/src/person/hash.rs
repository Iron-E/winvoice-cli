use super::Person;
use core::hash::{Hash, Hasher};

impl Hash for Person<'_, '_, '_>
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}
