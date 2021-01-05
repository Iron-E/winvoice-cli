use crate::Wrapper;

use clinvoice_data::{Employee, Location, Organization};

use std::collections::HashMap;

pub trait CrudOrganization<'addr, 'contact_info, 'email, 'name, 'rep_title, WEmployee, WLocation> :
	Into<HashMap<&'rep_title str, WEmployee>> +
	Into<WLocation> +
	Wrapper<Organization<'name, 'rep_title>> +
where
	'addr  : 'contact_info,
	'email : 'contact_info,
	 WEmployee : Wrapper<Employee<'addr, 'contact_info, 'email>>,
	 WLocation : Wrapper<Location<'name>>,
{

}
