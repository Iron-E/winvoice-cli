use std::fs;

use clinvoice_adapter::data::Updatable;

use super::BincodeLocation;
use crate::data::{
	Error,
	Result,
};

impl Updatable for BincodeLocation<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.location)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}
