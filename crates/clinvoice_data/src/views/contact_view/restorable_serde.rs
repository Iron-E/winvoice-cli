use super::ContactView;
use crate::views::RestorableSerde;

impl RestorableSerde for ContactView
{
	fn restore(&mut self, original: &Self)
	{
		if let ContactView::Address {
			location,
			export: _,
		} = self
		{
			if let ContactView::Address {
				location: original_location,
				export: _,
			} = original
			{
				location.restore(original_location);
				return;
			}

			panic!(
				"`original` {} was not an {}!",
				stringify!(ContactView),
				stringify!(Address)
			)
		}
	}
}
