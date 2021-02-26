#[macro_export]
macro_rules! AdaptPerson
{
	($name: ident, $($store_life: lifetime)*) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Person,
		};

		/// # Summary
		///
		/// A wrapper around [`Person`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$($store_life),*>
		{
			pub person: Person,
			pub store: Store<$($store_life),*>,
		}

		impl<$($store_life),*> Into<Person> for $name<$($store_life),*>
		{
			fn into(self) -> Person
			{
				self.person
			}
		}

		impl<$($store_life),*> Into<Store<$($store_life),*>> for $name<$($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				self.store
			}
		}
	}
}
