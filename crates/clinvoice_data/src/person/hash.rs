use core::hash::{
	Hash,
	Hasher,
};

use super::Person;

impl Hash for Person
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.id.hash(state);
	}
}
