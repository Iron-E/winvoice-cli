#[macro_export]
macro_rules! newtype_person
{
	($name:ident) =>
	{
		use clinvoice_data::Person;

		/// # Summary
		///
		/// A wrapper around [`Job`] for use with TomlDB.
		pub struct $name<'contact_info, 'email, 'name, 'phone> (Person<'contact_info, 'email, 'name, 'phone>);

		impl<'contact_info, 'email, 'name, 'phone> From<Person<'contact_info, 'email, 'name, 'phone>>
		for $name<'contact_info, 'email, 'name, 'phone>
		{
			fn from(person: Person<'contact_info, 'email, 'name, 'phone>) -> Self
			{
				return $name (person);
			}
		}
	}
}
