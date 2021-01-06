use crate::{data::{CrudLocation, CrudOrganization, CrudPerson}, Wrapper};

use clinvoice_data::Employee;

pub trait CrudEmployee<'contact_info, 'email, 'name, 'phone, 'rep_title, CLoc, COrg, CPrsn> :
	Into<COrg> +
	Into<CPrsn> +
	Wrapper<Employee<'contact_info, 'email, 'phone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 CLoc  : CrudLocation<'name>,
	 COrg  : CrudOrganization<'contact_info, 'email, 'name, 'phone, 'rep_title, Self, CLoc, CPrsn>,
	 CPrsn : CrudPerson<'contact_info, 'email, 'name, 'phone>,
{

}
