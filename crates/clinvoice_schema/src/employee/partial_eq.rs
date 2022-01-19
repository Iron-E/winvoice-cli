use super::Employee;

impl PartialEq for Employee
{
	fn eq(&self, other: &Self) -> bool
	{
		self.organization == other.organization && self.person == other.person
	}
}
