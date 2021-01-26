use super::TomlEmployee;
use clinvoice_data::Person;
use std::error::Error;

impl<'email, 'name, 'phone> Into<Result<Person<'email, 'name, 'phone>, Box<dyn Error>>>
for TomlEmployee<'email, 'phone, '_, '_, '_, '_>
{
	fn into(self) -> Result<Person<'email, 'name, 'phone>, Box<dyn Error>>
	{
		todo!()
	}
}

