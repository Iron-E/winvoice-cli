use
{
	super::ContactView,
	crate::views::PreservableSerde,
};

impl PreservableSerde for ContactView
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

impl PreservableSerde for Vec<ContactView>
{
	fn restore(&mut self, original: &Self)
	{
		self.iter_mut().enumerate().for_each(|(index, contact)|
			if let Some(original_contact) = original.get(index)
			{
				contact.restore(original_contact)
			}
		);
	}
}
