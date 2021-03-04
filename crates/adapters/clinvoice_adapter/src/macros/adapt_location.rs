#[macro_export]
macro_rules! AdaptLocation
{
	($name: ident, $store_life: lifetime) =>
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
		pub struct $name<$store_life>
		{
			pub location: Location,
			pub store: &$store_life Store,
		}

		impl Into<Location> for $name<'_>
		{
			fn into(self) -> Location
			{
				self.location
			}
		}

		impl Into<Store> for $name<'_>
		{
			fn into(self) -> Store
			{
				self.store.clone()
			}
		}
	}
}
