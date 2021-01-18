use super::TomlJob;
use clinvoice_adapter::data::Updatable;
use clinvoice_data::chrono::TimeZone;
use std::error::Error;

impl<TZone> Updatable for TomlJob<'_, '_, '_, '_, '_, '_, '_, TZone> where TZone : TimeZone
{
	fn update<'err>(&self) -> Result<(), &'err dyn Error>
	{
		todo!()
	}
}
