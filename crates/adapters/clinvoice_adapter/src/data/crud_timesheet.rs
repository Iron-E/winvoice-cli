use clinvoice_data::{chrono::TimeZone, Employee, Job, Timesheet};

pub trait CrudTimesheet<'contact_info, 'email, 'objectives, 'notes, 'phone, 'work_notes, TZone> :
	From<Timesheet<'work_notes, TZone>> + Into<Timesheet<'work_notes, TZone>> +
	Into<Employee<'contact_info, 'email, 'phone>> +
	Into<Job<'objectives, 'notes, TZone>> +
where
	'email : 'contact_info,
	'phone : 'contact_info,
	 TZone : TimeZone,
{

}
