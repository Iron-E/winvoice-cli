#[macro_export]
macro_rules! AdaptJob
{
	($name: ident, $store_life: lifetime) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Job,
		};

		/// # Summary
		///
		/// A wrapper around [`Job`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$store_life>
		{
			pub job: Job,
			pub store: &$store_life Store,
		}

		impl Into<Job> for $name<'_>
		{
			fn into(self) -> Job
			{
				self.job
			}
		}
	};
}
