use super::BincodeEmployee;
use clinvoice_adapter::data::Updatable;
use std::error::Error;

impl Updatable for BincodeEmployee<'_, '_, '_, '_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
