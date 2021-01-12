use clinvoice_data::{Employee, Location, Organization};

use std::collections::HashSet;

pub trait CrudOrganization<'contact_info, 'email, 'name, 'phone, 'title> :
	From<Organization<'name>> +
	Into<HashSet<Employee<'contact_info, 'email, 'phone, 'title>>> +
	Into<Location<'name>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
{

}
