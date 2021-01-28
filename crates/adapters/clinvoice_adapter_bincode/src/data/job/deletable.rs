use super::BincodeJob;
use clinvoice_adapter::data::Deletable;
use std::error::Error;

impl<'pass, 'path, 'user> Deletable<'pass, 'path, 'user> for BincodeJob<'_, '_, '_, '_, 'pass, 'path, 'user>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
