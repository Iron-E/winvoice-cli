use super::BincodeJob;
use clinvoice_adapter::data::Deletable;
use std::error::Error;

impl Deletable for BincodeJob<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
