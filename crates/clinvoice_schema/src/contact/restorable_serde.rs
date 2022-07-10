use super::{Contact, ContactKind};
use crate::{RestorableSerde, RestoreError, RestoreResult};

impl RestorableSerde for Contact
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		if let ContactKind::Address(ref mut location) = self.kind
		{
			match original.kind
			{
				ContactKind::Address(ref original_location) =>
				{
					location.try_restore(original_location)?
				},
				_ => return Err(RestoreError),
			}
		}

		Ok(())
	}
}
