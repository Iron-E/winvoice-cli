use
{
	super::Job,
	core::hash::{Hash, Hasher},
};

impl Hash for Job
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}

