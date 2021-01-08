#[macro_export]
macro_rules! NewtypeJob
{
	($name: ident, $($life: lifetime)*, $T: ident) =>
	{
		use clinvoice_data::{chrono::TimeZone, Job};

		/// # Summary
		///
		/// A wrapper around [`Job`] for use with TomlDB.
		pub struct $name<$($life),*, $T> (Job<$($life),*, $T>) where $T : TimeZone;

		impl<$($life),*, $T> From<Job<$($life),*, $T>> for $name<$($life),*, $T> where
			'work_notes : 'timesheets,
			 $T : 'timesheets + TimeZone,
		{
			fn from(job: Job<$($life),*, $T>) -> Self
			{
				return $name (job);
			}
		}
	};
}
