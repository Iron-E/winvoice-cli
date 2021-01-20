use super::TomlJob;
use clinvoice_data::{chrono::TimeZone, Organization};
use std::error::Error;

impl<'name, TZone> Into<Result<Organization<'name>, Box<dyn Error>>>
for TomlJob<'_, '_, '_, '_, '_, '_, '_, TZone>
where TZone : TimeZone
{
	fn into(self) -> Result<Organization<'name>, Box<dyn Error>>
	{
		todo!()
	}
}

