#[macro_export]
macro_rules! AdaptPerson
{
	($name: ident, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Person;
		use core::ops::Deref;

		/// # Summary
		///
		/// A wrapper around [`Person`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, PartialEq)]
		pub struct $name<$($store_life),*>
		{
			person: Person,
			pub store: Store<$($store_life),*>,
		}

		impl<$($store_life),*> Deref for $name<$($store_life),*>
		{
			type Target = Person;

			fn deref(&self) -> &Self::Target
			{
				return &self.person;
			}
		}

		impl<$($store_life),*> Into<Person> for $name<$($store_life),*>
		{
			fn into(self) -> Person
			{
				return self.person;
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
