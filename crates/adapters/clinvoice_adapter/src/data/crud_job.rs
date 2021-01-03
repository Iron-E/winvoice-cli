use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Job};

pub trait CrudJob<'objectives, 'notes, 'timesheets, 'timesheet_note, TZone, W> where
	'timesheet_note : 'timesheets,
	TZone           : 'timesheets + TimeZone,
	W               : Wrapper<Job<'objectives, 'notes, 'timesheets, 'timesheet_note, TZone>>,
{

}
