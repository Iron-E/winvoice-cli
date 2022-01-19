use super::Employee;
use crate::RestorableSerde;

impl RestorableSerde for Employee
{
	fn restore(&mut self, original: &Self)
	{
		self.contact_info.restore(&original.contact_info);
		self.id = original.id;
		self.organization.restore(&original.organization);
		self.person.restore(&original.person);
	}
}
