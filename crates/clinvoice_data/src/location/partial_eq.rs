use super::Location;

impl PartialEq for Location
{
	fn eq(&self, other: &Self) -> bool
	{
		self.id == other.id
	}
}
