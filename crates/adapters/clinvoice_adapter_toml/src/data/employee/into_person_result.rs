use super::TomlEmployee;
use clinvoice_data::Person;
use std::error::Error;

impl<'contact_info, 'email, 'err, 'name, 'phone> Into<Result<Person<'contact_info, 'email, 'name, 'phone>, &'err dyn Error>>
for TomlEmployee<'contact_info, 'email, 'phone, '_, '_, '_, '_>
{
	fn into(self) -> Result<Person<'contact_info, 'email, 'name, 'phone>, &'err dyn Error>
	{
		todo!()
	}
}

