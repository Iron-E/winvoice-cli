use super::BincodeOrganization;
use clinvoice_data::Employee;
use std::{collections::HashSet, error::Error};

impl Into<Result<HashSet<Employee>, Box<dyn Error>>> for BincodeOrganization<'_, '_, '_>
{
	fn into(self) -> Result<HashSet<Employee>, Box<dyn Error>>
	{
		todo!()
	}
}
