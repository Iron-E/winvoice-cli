use clinvoice_data::Person;

pub trait CrudPerson<'contact_info, 'email, 'name, 'phone> : From<Person<'contact_info, 'email, 'name, 'phone>> where
	'email : 'contact_info,
	'phone : 'contact_info,
{

}
