use std::fs;

use clinvoice_adapter::data::Updatable;

use super::BincodePerson;
use crate::data::{Error, Result};

impl Updatable for BincodePerson<'_, '_>
{
	type Error = Error;

	fn update(&self) -> Result<()>
	{
		let serialized = bincode::serialize(&self.person)?;
		fs::write(self.filepath(), serialized)?;
		Ok(())
	}
}
