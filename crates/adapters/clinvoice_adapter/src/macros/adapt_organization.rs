#[macro_export]
macro_rules! AdaptOrganization
{
	($name: ident, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Organization;
		use core::ops::Deref;

		/// # Summary
		///
		/// A wrapper around [`Organization`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, PartialEq)]
		pub struct $name<$($store_life),*>
		{
			organization: Organization,
			pub store: Store<$($store_life),*>,
		}

		impl<$($store_life),*> Deref for $name<$($store_life),*>
		{
			type Target = Organization;

			fn deref(&self) -> &Self::Target
			{
				return &self.organization;
			}
		}

		impl<$($store_life),*> Into<Organization> for $name<$($store_life),*>
		{
			fn into(self) -> Organization
			{
				return self.organization;
			}
		}

		impl<$($store_life),*> Into<Store<$($store_life),*>> for $name<$($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	}
}
