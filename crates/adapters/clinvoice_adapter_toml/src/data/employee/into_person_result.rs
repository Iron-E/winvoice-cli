use super::TomlEmployee;
use clinvoice_data::Person;
use std::error::Error;

impl<'contact_info, 'email, 'name, 'phone> Into<Result<Person<'contact_info, 'email, 'name, 'phone>, Box<dyn Error>>>
for TomlEmployee<'contact_info, 'email, 'phone, '_, '_, '_, '_>
{
	fn into(self) -> Result<Person<'contact_info, 'email, 'name, 'phone>, Box<dyn Error>>
	{
		todo!()
	}
}

