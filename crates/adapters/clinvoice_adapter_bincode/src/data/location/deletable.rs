use super::BincodeLocation;
use clinvoice_adapter::data::Deletable;
use std::error::Error;

impl Deletable for BincodeLocation<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
