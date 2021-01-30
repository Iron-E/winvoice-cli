#[macro_export]
macro_rules! AdaptLocation
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Location;
		use core::ops::Deref;

		/// # Summary
		///
		/// A wrapper around [`Location`] with a [`Store`] that points to its location.
		#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			pub location: Location<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Deref for $name<$($life),*, $($store_life),*>
		{
			type Target = Location<$($life),*>;

			fn deref(&self) -> &Self::Target
			{
				return &self.location;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Location<$($life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Location<$($life),*>
			{
				return self.location;
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
