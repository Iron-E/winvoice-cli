use super::EmployeeView;

impl PartialEq for EmployeeView
{
	fn eq(&self, other: &Self) -> bool
	{
		self.organization == other.organization && self.person == other.person
	}
}
