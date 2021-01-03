use crate::Wrapper;

use clinvoice_data::Person;

pub trait CrudPerson<'addr, 'contact_info, 'email, 'name, W> where
	'addr  : 'contact_info,
	'email : 'contact_info,
	 W     :  Wrapper<Person<'addr, 'contact_info, 'email, 'name>>,
{

}
