#[macro_export]
macro_rules! NewtypePerson
{
	($name: ident, $($life: lifetime)*) =>
	{
		use clinvoice_data::Person;

		/// # Summary
		///
		/// A wrapper around [`Job`] for use with TomlDB.
		pub struct $name<$($life),*> (Person<$($life),*>);

		impl<$($life),*> From<Person<$($life),*>> for $name<$($life),*> where
			'email : 'contact_info,
			'phone : 'contact_info,
		{
			fn from(person: Person<$($life),*>) -> Self
			{
				return $name (person);
			}
		}
	}
}
