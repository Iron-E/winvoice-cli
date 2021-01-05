use crate::Wrapper;

use clinvoice_data::{Employee, Organization, Person};

pub trait CrudEmployee<'addr, 'contact_info, 'email, 'name, 'rep_title, WOrganization, WPerson> :
	Into<WOrganization> +
	Into<WPerson> +
	Wrapper<Employee<'addr, 'contact_info, 'email>> +
where
	'addr  : 'contact_info,
	'email : 'contact_info,
	 WOrganization : Wrapper<Organization<'name, 'rep_title>>,
	 WPerson       : Wrapper<Person<'addr, 'contact_info, 'email, 'name>>,
{

}
