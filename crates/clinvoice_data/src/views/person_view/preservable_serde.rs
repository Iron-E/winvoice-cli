use
{
	super::PersonView,
	crate::views::PreservableSerde,
};

impl PreservableSerde for PersonView
{
	fn restore(&mut self, original: &Self)
	{
		self.contact_info.restore(&original.contact_info);
		self.id = original.id;
	}
}
