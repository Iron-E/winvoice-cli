#[macro_export]
macro_rules! NewtypeJob
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*, $T: ident) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::{chrono::TimeZone, Job};

		/// # Summary
		///
		/// A wrapper around [`Job`] for use with TomlDB.
		#[derive(Debug)]
		pub struct $name<$($life),*, $($store_life),*, $T> where
			'work_notes : 'timesheets,
			 $T : 'timesheets + TimeZone,
		{
			pub job: Job<$($life),*, $T>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*, $T> Into<Job<$($life),*, $T>> for $name<$($life),*, $($store_life),*, $T> where
			'work_notes : 'timesheets,
			 $T : 'timesheets + TimeZone,
		{
			fn into(self) -> Job<$($life),*, $T>
			{
				return self.job;
			}
		}

		impl<$($life),*, $($store_life),*, $T> Into<Store<$($store_life),*>> for $name<$($life),*, $($store_life),*, $T> where
			'work_notes : 'timesheets,
			 $T : 'timesheets + TimeZone,
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	};
}
