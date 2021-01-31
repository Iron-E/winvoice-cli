use super::Employee;
use core::hash::{Hash, Hasher};

impl Hash for Employee
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
		self.organization_id.hash(state);
		self.person_id.hash(state);
	}
}
