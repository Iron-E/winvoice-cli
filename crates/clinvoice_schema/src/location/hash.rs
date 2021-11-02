use core::hash::{Hash, Hasher};

use super::Location;

impl Hash for Location
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.id.hash(state);
	}
}
