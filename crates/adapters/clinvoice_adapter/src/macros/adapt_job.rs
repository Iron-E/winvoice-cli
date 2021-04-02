#[macro_export]
macro_rules! AdaptJob
{
	($name: ident, $job_life: lifetime, $store_life: lifetime) =>
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
		pub struct $name<$job_life, $store_life>
		{
			pub job: &$job_life Job,
			pub store: &$store_life Store,
		}
	};
}
