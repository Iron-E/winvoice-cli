use
{
	super::PersonView,
	core::hash::{Hash, Hasher},
};

impl Hash for PersonView
{
	fn hash<H>(&self, state: &mut H) where H : Hasher
	{
		self.id.hash(state);
	}
}
