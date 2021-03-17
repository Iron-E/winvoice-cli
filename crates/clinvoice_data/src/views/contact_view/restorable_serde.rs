use
{
	super::ContactView,
	crate::views::RestorableSerde,
	std::{collections::HashMap, hash::Hash},
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

impl<S> RestorableSerde for HashMap<S, ContactView> where
	S : Eq + Hash
{
	fn restore(&mut self, original: &Self)
	{
		self.iter_mut().for_each(|(label, contact)|
			if let Some(original_contact) = original.get(label)
			{
				contact.restore(original_contact)
			}
		);
	}
}
