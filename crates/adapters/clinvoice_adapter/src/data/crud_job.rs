use clinvoice_data::{chrono::TimeZone, Job, Organization};

pub trait CrudJob<'objectives, 'name, 'notes, 'rep_title, 'timesheets, 'work_notes, TZone> :
	From<Job<'objectives, 'notes, 'timesheets, 'work_notes, TZone>> +
	Into<Organization<'name, 'rep_title>> +
where
	 'work_notes : 'timesheets,
	  TZone : 'timesheets + TimeZone,
{

}
