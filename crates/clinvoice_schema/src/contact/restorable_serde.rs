use super::Contact;
use crate::{ContactKind, RestorableSerde, RestoreError, RestoreResult};

impl RestorableSerde for Contact
{
	fn try_restore(&mut self, original: &Self) -> RestoreResult<()>
	{
		self.employee_id = original.employee_id;
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
