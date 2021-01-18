use super::TomlPerson;
use clinvoice_adapter::data::Updatable;
use std::error::Error;

impl Updatable for TomlPerson<'_, '_, '_, '_, '_, '_, '_>
{
	fn update<'err>(&self) -> Result<(), &'err dyn Error>
	{
		todo!()
	}
}
