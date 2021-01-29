use super::BincodeOrganization;
use clinvoice_adapter::data::Updatable;
use std::{error::Error, fs};

impl Updatable for BincodeOrganization<'_, '_, '_, '_>
{
	fn update(&self) -> Result<(), Box<dyn Error>>
	{
		fs::write(self.filepath(), bincode::serialize(&self.organization)?)?;
		return Ok(());
	}
}
