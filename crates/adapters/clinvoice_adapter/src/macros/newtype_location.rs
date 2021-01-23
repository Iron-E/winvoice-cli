#[macro_export]
macro_rules! NewtypeLocation
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Location;

		/// # Summary
		///
		/// A wrapper around [`Location`] for use with TomlDB.
		#[derive(Debug)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			pub location: Location<$($life),*>,
			pub store: Store<$($store_life),*>,
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
