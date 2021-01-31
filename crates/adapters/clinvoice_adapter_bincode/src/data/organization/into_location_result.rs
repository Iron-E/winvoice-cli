use super::BincodeOrganization;
use clinvoice_data::Location;
use std::error::Error;

impl Into<Result<Location, Box<dyn Error>>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> Result<Location, Box<dyn Error>>
	{
		todo!()
	}
}

