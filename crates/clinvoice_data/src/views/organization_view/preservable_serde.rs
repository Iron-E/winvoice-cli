use
{
	super::OrganizationView,
	crate::views::PreservableSerde,
};

impl PreservableSerde for OrganizationView
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
		self.location.restore(&original.location);
	}
}
