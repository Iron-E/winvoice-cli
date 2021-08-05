use std::fs;

use clinvoice_adapter::data::Updatable;

use super::BincodeJob;
use crate::data::{Error, Result};

impl Updatable for BincodeJob<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.job)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}
