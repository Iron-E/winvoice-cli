use super::EmployeeView;
use crate::views::RestorableSerde;

impl RestorableSerde for EmployeeView
{
	fn restore(&mut self, original: &Self)
	{
		self.contact_info.restore(&original.contact_info);
		self.id = original.id;
		self.organization.restore(&original.organization);
		self.person.restore(&original.person);
	}
}
