use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Employee, Timesheet};

pub trait CrudTimesheet<'addr, 'contact_info, 'email, 'work_notes, TZone, WEmployee> :
	Into<WEmployee> +
	Wrapper<Timesheet<'work_notes, TZone>> +
where
	'addr  : 'contact_info,
	'email : 'contact_info,
	 TZone     : TimeZone,
	 WEmployee : Wrapper<Employee<'addr, 'contact_info, 'email>>,
{

}
