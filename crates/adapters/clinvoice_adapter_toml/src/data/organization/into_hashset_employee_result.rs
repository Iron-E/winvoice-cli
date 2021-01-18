use super::TomlOrganization;
use clinvoice_data::Employee;
use std::{collections::HashSet, error::Error};

impl<'contact_info, 'email, 'err, 'phone, 'title> Into<Result<HashSet<Employee<'contact_info, 'email, 'phone, 'title>>, &'err dyn Error>>
for TomlOrganization<'_, '_, '_, '_>
{
	fn into(self) -> Result<HashSet<Employee<'contact_info, 'email, 'phone, 'title>>, &'err dyn Error>
	{
		todo!()
	}
}
