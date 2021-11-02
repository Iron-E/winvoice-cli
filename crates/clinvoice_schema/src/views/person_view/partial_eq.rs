use super::PersonView;

impl PartialEq for PersonView
{
	fn eq(&self, other: &Self) -> bool
	{
		self.id == other.id
	}
}
