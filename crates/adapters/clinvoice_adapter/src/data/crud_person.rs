use crate::Wrapper;

use clinvoice_data::Person;

pub trait CrudPerson<'addr, 'contact_info, 'email, 'name>
	: Wrapper<Person<'addr, 'contact_info, 'email, 'name>>
where
	'addr  : 'contact_info,
	'email : 'contact_info,
{

}
