use super::Organization;
use core::hash::{Hash, Hasher};

impl Hash for Organization
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}

