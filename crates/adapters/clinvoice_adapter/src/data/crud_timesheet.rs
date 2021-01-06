use crate::{data::{CrudEmployee, CrudJob, CrudLocation, CrudOrganization, CrudPerson}, Wrapper};

use clinvoice_data::{chrono::TimeZone, Timesheet};

pub trait CrudTimesheet<'contact_info, 'email, 'name, 'objectives, 'notes, 'phone, 'rep_title, 'work_notes, CEmp, CLoc, CJob, COrg, CPrsn, TZone> :
	Into<CEmp> +
	Into<CJob> +
	Wrapper<Timesheet<'work_notes, TZone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 CEmp  : CrudEmployee<'contact_info, 'email, 'name, 'phone, 'rep_title, CLoc, COrg, CPrsn>,
	 CJob  : CrudJob<'contact_info, 'email, 'objectives, 'name, 'notes, 'phone, 'rep_title, COrg, CEmp, CLoc, CPrsn, TZone>,
	 CLoc  : CrudLocation<'name>,
	 COrg  : CrudOrganization<'contact_info, 'email, 'name, 'phone, 'rep_title, CEmp, CLoc, CPrsn>,
	 CPrsn : CrudPerson<'contact_info, 'email, 'name, 'phone>,
	 TZone : TimeZone,
{

}
