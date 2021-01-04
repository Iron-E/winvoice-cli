use crate::Wrapper;

use clinvoice_data::{chrono::TimeZone, Timesheet};

pub trait CrudTimesheet<'work_notes, TZone> : Wrapper<Timesheet<'work_notes, TZone>> where TZone : TimeZone
{

}
