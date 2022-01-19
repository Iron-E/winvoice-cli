use core::hash::{Hash, Hasher};

use super::Employee;

impl Hash for Employee
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.organization.hash(state);
		self.person.hash(state);
	}
}
