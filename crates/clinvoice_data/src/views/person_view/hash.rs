use core::hash::{Hash, Hasher};

use super::PersonView;

impl Hash for PersonView
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.id.hash(state);
	}
}
