use super::Employee;

impl PartialEq for Employee
{
	fn eq(&self, other: &Self) -> bool
	{
		self.organization_id == other.organization_id && self.person_id == other.person_id
	}
}
