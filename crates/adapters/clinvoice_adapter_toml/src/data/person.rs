use clinvoice_data::Person;

/// # Summary
///
/// A wrapper around [`Person`] for use with TomlDB.
pub struct TomlPerson<'addr, 'contact_info, 'email, 'name>
(
	Person<'addr, 'contact_info, 'email, 'name>,
);
