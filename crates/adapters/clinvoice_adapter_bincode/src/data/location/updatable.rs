use super::BincodeLocation;
use clinvoice_adapter::data::Updatable;
use std::{error::Error, fs};
use bincode;

impl Updatable for BincodeLocation<'_, '_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.location)?)?;
		return Ok(());
	}
}
