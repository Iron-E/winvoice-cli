use
{
	core::hash::{Hash, Hasher},

	super::Employee,
};

impl Hash for Employee
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.organization_id.hash(state);
		self.person_id.hash(state);
	}
}
