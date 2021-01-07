#[macro_export]
macro_rules! NewtypeTimesheet
{
	($name: ident, $($life: lifetime)*, $T: ident) =>
	{
		use clinvoice_data::{chrono::TimeZone, Timesheet};

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<$($life),*, $T> (Timesheet<$($life),*, $T>) where $T : TimeZone;

		impl<$($life),*, $T> From<Timesheet<$($life),*, $T>> for $name<$($life),*, $T> where
			$T : TimeZone
		{
			fn from(timesheet: Timesheet<$($life),*, $T>) -> Self
			{
				return $name (timesheet);
			}
		}
	}
}

