#[macro_export]
macro_rules! AdaptLocation
{
	($name: ident, $loc_life: lifetime, $store_life: lifetime) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Location,
		};

		/// # Summary
		///
		/// A wrapper around [`Location`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$loc_life, $store_life>
		{
			pub location: &$loc_life Location,
			pub store: &$store_life Store,
		}
	}
}
