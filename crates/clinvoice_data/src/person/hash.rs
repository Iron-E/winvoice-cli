use
{
	super::Person,
	core::hash::{Hash, Hasher},
};

impl Hash for Person
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}
