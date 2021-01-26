use super::TomlPerson;
use clinvoice_adapter::data::Updatable;
use std::error::Error;

impl Updatable for TomlPerson<'_, '_, '_, '_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
