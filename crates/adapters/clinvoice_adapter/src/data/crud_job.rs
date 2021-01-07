use clinvoice_data::{chrono::TimeZone, Job, Organization};

pub trait CrudJob<'objectives, 'name, 'notes, 'rep_title, TZone> :
	From<Job<'objectives, 'notes, TZone>> +
	Into<Organization<'name, 'rep_title>> +
where
	 TZone : TimeZone,
{

}
