use
{
	super::LocationView,
	crate::views::PreservableSerde,
};

impl PreservableSerde for LocationView
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
	}
}
