use super::TomlOrganization;
use clinvoice_adapter::data::Updatable;
use std::error::Error;

impl Updatable for TomlOrganization<'_, '_, '_, '_>
{
	fn update<'err>(&self) -> Result<(), &'err dyn Error>
	{
		todo!()
	}
}
