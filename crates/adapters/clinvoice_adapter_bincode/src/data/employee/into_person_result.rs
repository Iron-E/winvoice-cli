use super::BincodeEmployee;
use clinvoice_data::Person;
use std::error::Error;

impl Into<Result<Person, Box<dyn Error>>> for BincodeEmployee<'_, '_, '_>
{
	fn into(self) -> Result<Person, Box<dyn Error>>
	{
		todo!()
	}
}

