use super::Person;

impl PartialEq for Person
{
	fn eq(&self, other: &Self) -> bool
	{
		self.id == other.id
	}
}
