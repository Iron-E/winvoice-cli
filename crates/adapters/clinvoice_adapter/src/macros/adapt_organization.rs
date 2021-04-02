#[macro_export]
macro_rules! AdaptOrganization
{
	($name: ident, $org_life: lifetime, $store_life: lifetime) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Organization,
		};

		/// # Summary
		///
		/// A wrapper around [`Organization`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$org_life, $store_life>
		{
			pub organization: &$org_life Organization,
			pub store: &$store_life Store,
		}
	}
}
