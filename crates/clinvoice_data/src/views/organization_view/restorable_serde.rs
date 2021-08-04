use super::OrganizationView;
use crate::views::RestorableSerde;

impl RestorableSerde for OrganizationView
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
		self.location.restore(&original.location);
	}
}
