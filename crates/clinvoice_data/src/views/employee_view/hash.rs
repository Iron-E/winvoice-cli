use
{
	core::hash::{Hash, Hasher},

	super::EmployeeView,
};

impl Hash for EmployeeView
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.organization.hash(state);
		self.person.hash(state);
	}
}
