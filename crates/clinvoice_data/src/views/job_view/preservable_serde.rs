use
{
	super::JobView,
	crate::views::PreservableSerde,
};

impl PreservableSerde for JobView
{
	fn restore(&mut self, original: &Self)
	{
		self.client.restore(&original.client);
		self.id = original.id;
	}
}
