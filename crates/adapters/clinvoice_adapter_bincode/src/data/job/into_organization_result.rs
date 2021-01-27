use super::BincodeJob;
use clinvoice_data::Organization;
use std::error::Error;

impl<'name> Into<Result<Organization<'name>, Box<dyn Error>>> for BincodeJob<'_, '_, '_, '_, '_, '_>
{
	fn into(self) -> Result<Organization<'name>, Box<dyn Error>>
	{
		todo!()
	}
}

