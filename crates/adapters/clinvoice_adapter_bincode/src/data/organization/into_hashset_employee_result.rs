use super::BincodeOrganization;
use clinvoice_data::Employee;
use std::{collections::BTreeSet, error::Error};

impl<'email, 'phone, 'title> Into<Result<BTreeSet<Employee<'email, 'phone, 'title>>, Box<dyn Error>>>
for BincodeOrganization<'_, '_, '_, '_>
{
	fn into(self) -> Result<BTreeSet<Employee<'email, 'phone, 'title>>, Box<dyn Error>>
	{
		todo!()
	}
}
