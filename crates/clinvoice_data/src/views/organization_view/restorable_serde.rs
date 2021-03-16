use
{
	super::OrganizationView,
	crate::views::RestorableSerde,
};

impl RestorableSerde for OrganizationView
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
		self.location.restore(&original.location);
	}
}
