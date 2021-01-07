use clinvoice_data::{Employee, Location, Organization};

use std::collections::HashMap;

pub trait CrudOrganization<'contact_info, 'email, 'name, 'phone, 'rep_title> :
	From<Organization<'name, 'rep_title>> +
	Into<HashMap<&'rep_title str, Employee<'contact_info, 'email, 'phone>>> +
	Into<Location<'name>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
{

}
