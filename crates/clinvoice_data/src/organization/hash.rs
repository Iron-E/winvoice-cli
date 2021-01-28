use super::Organization;
use core::hash::{Hash, Hasher};

impl Hash for Organization<'_>
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}

