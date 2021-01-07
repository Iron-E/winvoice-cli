#[macro_export]
macro_rules! newtype_timesheet
{
	($name:ident) =>
	{
		use clinvoice_data::{chrono::TimeZone, Timesheet};

		/// # Summary
		///
		/// Wrapper around [`Employee`].
		pub struct $name<'work_notes, TZone> (Timesheet<'work_notes, TZone>) where TZone : TimeZone;

		impl<'work_notes, TZone> From<Timesheet<'work_notes, TZone>>
		for $name<'work_notes, TZone>
		where TZone : TimeZone
		{
			fn from(timesheet: Timesheet<'work_notes, TZone>) -> Self
			{
				return $name (timesheet);
			}
		}
	}
}

