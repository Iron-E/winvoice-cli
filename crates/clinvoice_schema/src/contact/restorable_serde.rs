use super::Contact;
use crate::{RestorableSerde, RestoreError, RestoreResult};

impl RestorableSerde for Contact
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		if let Contact::Address {
			label: _,
			location,
			export: _,
		} = self
		{
			match original
			{
				Contact::Address {
					label: _,
					location: original_location,
					export: _,
				} => location.try_restore(original_location)?,

				_ => return Err(RestoreError),
			}
		}

		Ok(())
	}
}
