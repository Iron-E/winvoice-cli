#[macro_export]
macro_rules! AdaptOrganization
{
	($name: ident, $store_life: lifetime) =>
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
		pub struct $name<$store_life>
		{
			pub organization: Organization,
			pub store: &$store_life Store,
		}

		impl Into<Organization> for $name<'_>
		{
			fn into(self) -> Organization
			{
				self.organization
			}
		}
	}
}
