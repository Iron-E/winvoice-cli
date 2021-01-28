use super::BincodeOrganization;
use clinvoice_adapter::data::Deletable;
use std::error::Error;

impl<'pass, 'path, 'user> Deletable<'pass, 'path, 'user> for BincodeOrganization<'_, 'pass, 'path, 'user>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
