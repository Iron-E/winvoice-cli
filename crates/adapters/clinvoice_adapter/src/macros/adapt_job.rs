#[macro_export]
macro_rules! AdaptJob
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Job;

		/// # Summary
		///
		/// A wrapper around [`Job`] for use with TomlDB.
		#[derive(Debug)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			pub job: Job<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Into<Job<$($life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Job<$($life),*>
			{
				return self.job;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Store<$($store_life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	};
}
