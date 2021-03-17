use
{
	super::ContactView,
	crate::views::RestorableSerde,
};

impl RestorableSerde for ContactView
{
	fn restore(&mut self, original: &Self)
	{
		if let ContactView::Address(location) = self
		{
			if let ContactView::Address(original_location) = original
			{
				location.restore(original_location);
			}
			else
			{
				panic!("`original` {} was not an {}!", stringify!(ContactView), stringify!(Address))
			}
		}
	}
}
