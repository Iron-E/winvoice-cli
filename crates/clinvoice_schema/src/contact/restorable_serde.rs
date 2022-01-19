use super::Contact;
use crate::RestorableSerde;

impl RestorableSerde for Contact
{
	fn restore(&mut self, original: &Self)
	{
		if let Contact::Address {
			location,
			export: _,
		} = self
		{
			if let Contact::Address {
				location: original_location,
				export: _,
			} = original
			{
				location.restore(original_location);
				return;
			}

			panic!("`original` Contact was not an Address!")
		}
	}
}
