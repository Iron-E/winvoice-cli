use
{
	super::PersonView,
	crate::views::RestorableSerde,
};

impl RestorableSerde for PersonView
{
	fn restore(&mut self, original: &Self)
	{
		self.id = original.id;
	}
}
