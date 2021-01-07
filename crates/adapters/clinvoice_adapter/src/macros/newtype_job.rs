#[macro_export]
macro_rules! newtype_job
{
	($name:ident) =>
	{
		use clinvoice_data::{chrono::TimeZone, Job};

		/// # Summary
		///
		/// A wrapper around [`Job`] for use with TomlDB.
		pub struct $name<'objectives, 'notes, TZone> (Job<'objectives, 'notes, TZone>) where TZone : TimeZone;

		impl<'objectives, 'notes, TZone> From<Job<'objectives, 'notes, TZone>>
		for $name<'objectives, 'notes, TZone>
		where TZone : TimeZone
		{
			fn from(job: Job<'objectives, 'notes, TZone>) -> Self
			{
				return $name (job);
			}
		}
	}
}
