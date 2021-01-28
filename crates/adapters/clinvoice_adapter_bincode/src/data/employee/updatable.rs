use super::BincodeEmployee;
use clinvoice_adapter::data::Updatable;
use std::{error::Error, fs};
use bincode;

impl Updatable for BincodeEmployee<'_, '_, '_, '_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.employee)?)?;
		return Ok(());
	}
}