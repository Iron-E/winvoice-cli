use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Job};

pub trait CrudJob<'objectives, 'names, 'notes, 'rep_title, 'timesheets, 'timesheet_note, TZone> :
	Wrapper<Job<'objectives, 'names, 'notes, 'rep_title, 'timesheets, 'timesheet_note, TZone>> +
where
	'timesheet_note : 'timesheets,
	 TZone          : 'timesheets + TimeZone,
{

}
