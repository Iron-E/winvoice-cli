#[macro_export]
macro_rules! AdaptOrganization
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Organization;

		/// # Summary
		///
		/// A wrapper around [`Organization`] with a [`Store`] that points to its location.
		#[derive(Debug)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			pub organization: Organization<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Into<Organization<$($life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Organization<$($life),*>
			{
				return self.organization;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Store<$($store_life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	}
}
