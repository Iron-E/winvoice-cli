use super::TomlJob;
use clinvoice_data::{chrono::TimeZone, Organization};
use std::error::Error;

impl<'err, 'name, TZone> Into<Result<Organization<'name>, &'err dyn Error>>
for TomlJob<'_, '_, '_, '_, '_, '_, '_, TZone>
where TZone : TimeZone
{
	fn into(self) -> Result<Organization<'name>, &'err dyn Error>
	{
		todo!()
	}
}

