use
{
	super::EmployeeView,
	core::hash::{Hash, Hasher},
};

impl Hash for EmployeeView
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.organization.hash(state);
		self.person.hash(state);
	}
}
