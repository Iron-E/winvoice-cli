use crate::{data::{CrudEmployee, CrudLocation, CrudPerson}, Wrapper};

use clinvoice_data::Organization;

use std::collections::HashMap;

pub trait CrudOrganization<'contact_info, 'email, 'name, 'phone, 'rep_title, CEmp, CLoc, CPrsn> :
	Into<CLoc> +
	Into<HashMap<&'rep_title str, CEmp>> +
	Wrapper<Organization<'name, 'rep_title>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 CEmp  : CrudEmployee<'contact_info, 'email, 'name, 'phone, 'rep_title, CLoc, Self, CPrsn>,
	 CLoc  : CrudLocation<'name>,
	 CPrsn : CrudPerson<'contact_info, 'email, 'name, 'phone>,
{

}
