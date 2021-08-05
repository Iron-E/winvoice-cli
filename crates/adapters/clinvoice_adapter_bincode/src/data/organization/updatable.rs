use std::fs;

use clinvoice_adapter::data::Updatable;

use super::BincodeOrganization;
use crate::data::{Error, Result};

impl Updatable for BincodeOrganization<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.organization)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}
