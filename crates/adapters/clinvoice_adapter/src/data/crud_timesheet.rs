use clinvoice_data::{chrono::TimeZone, Employee, Timesheet};

pub trait CrudTimesheet<'contact_info, 'email, 'objectives, 'notes, 'phone, 'work_notes, TZone> :
	From<Timesheet<'work_notes, TZone>> + Into<Timesheet<'work_notes, TZone>> +
	Into<Employee<'contact_info, 'email, 'phone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 TZone : TimeZone,
{

}
