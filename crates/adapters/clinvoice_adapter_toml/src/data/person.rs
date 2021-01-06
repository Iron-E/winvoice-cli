use clinvoice_data::Person;

/// # Summary
///
/// A wrapper around [`Person`] for use with TomlDB.
pub struct TomlPerson<'contact_info, 'email, 'name, 'phone>
(
	Person<'contact_info, 'email, 'name, 'phone>,
);
