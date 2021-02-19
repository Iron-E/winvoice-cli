use
{
	super::Location,
	core::hash::{Hash, Hasher},
};

impl Hash for Location
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}

