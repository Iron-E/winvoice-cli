use
{
	core::hash::{Hash, Hasher},

	super::Job,
};

impl Hash for Job
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}

