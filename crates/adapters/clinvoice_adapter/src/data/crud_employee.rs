use clinvoice_data::{Employee, Organization, Person};

pub trait CrudEmployee<'contact_info, 'email, 'name, 'phone, 'rep_title> :
	From<Employee<'contact_info, 'email, 'phone>> +
	Into<Organization<'name, 'rep_title>> +
	Into<Person<'contact_info, 'email, 'name, 'phone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
{

}
