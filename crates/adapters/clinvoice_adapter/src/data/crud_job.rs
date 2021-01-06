use crate::{data::{CrudEmployee, CrudLocation, CrudOrganization, CrudPerson}, Wrapper};

use clinvoice_data::{chrono::TimeZone, Job};

pub trait CrudJob<'contact_info, 'email, 'objectives, 'name, 'notes, 'phone, 'rep_title, COrg, CEmp, CLoc, CPrsn, TZone> :
	Into<COrg> +
	Wrapper<Job<'objectives, 'notes, TZone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 CEmp  : CrudEmployee<'contact_info, 'email, 'name, 'phone, 'rep_title, CLoc, COrg, CPrsn>,
	 CLoc  : CrudLocation<'name>,
	 COrg  : CrudOrganization<'contact_info, 'email, 'name, 'phone, 'rep_title, CEmp, CLoc, CPrsn>,
	 CPrsn : CrudPerson<'contact_info, 'email, 'name, 'phone>,
	 TZone : TimeZone,
{

}
