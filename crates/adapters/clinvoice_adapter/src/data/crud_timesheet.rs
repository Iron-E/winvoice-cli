use clinvoice_data::{chrono::TimeZone, Employee, Timesheet};

pub trait CrudTimesheet<'contact_info, 'email, 'objectives, 'notes, 'phone, 'title, 'work_notes, TZone> :
	From<Timesheet<'work_notes, TZone>> +
	Into<Employee<'contact_info, 'email, 'phone, 'title>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 TZone :  TimeZone,
{

}
