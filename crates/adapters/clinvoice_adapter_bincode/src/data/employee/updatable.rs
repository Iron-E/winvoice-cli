use std::fs;

use clinvoice_adapter::data::Updatable;

use super::BincodeEmployee;
use crate::data::{Error, Result};

impl Updatable for BincodeEmployee<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.employee)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}
