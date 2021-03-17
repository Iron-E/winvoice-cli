#[macro_export]
macro_rules! AdaptPerson
{
	($name: ident, $per_life: lifetime, $store_life: lifetime) =>
	{
		use
		{
			clinvoice_adapter::Store,
			clinvoice_data::Person,
		};

		/// # Summary
		///
		/// A wrapper around [`Person`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, PartialEq)]
		pub struct $name<$per_life, $store_life>
		{
			pub person: &$per_life Person,
			pub store: &$store_life Store,
		}

		impl Into<Person> for $name<'_, '_>
		{
			fn into(self) -> Person
			{
				self.person.clone()
			}
		}
	}
}
