use super::BincodeOrganization;
use clinvoice_adapter::data::Deletable;
use std::error::Error;

impl Deletable for BincodeOrganization<'_, '_, '_>
{
	fn delete(&self, cascade: bool) -> Result<(), Box<dyn Error>>
	{
		todo!()
	}
}
