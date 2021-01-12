use clinvoice_data::{Employee, Organization, Person};

pub trait CrudEmployee<'contact_info, 'email, 'name, 'phone, 'title> :
	From<Employee<'contact_info, 'email, 'phone, 'title>> +
	Into<Organization<'name>> +
	Into<Person<'contact_info, 'email, 'name, 'phone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
{

}
