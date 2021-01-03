use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Timesheet};

pub trait CrudTimesheet<'work_notes, TZone, W> where
	TZone : TimeZone,
	W     : Wrapper<Timesheet<'work_notes, TZone>>,
{

}
