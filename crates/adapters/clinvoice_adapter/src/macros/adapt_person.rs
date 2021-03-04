#[macro_export]
macro_rules! AdaptPerson
{
	($name: ident, $store_life: lifetime) =>
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
		pub struct $name<$store_life>
		{
			pub person: Person,
			pub store: &$store_life Store,
		}

		impl Into<Person> for $name<'_>
		{
			fn into(self) -> Person
			{
				self.person
			}
		}
	}
}
