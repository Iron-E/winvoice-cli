use clinvoice_data::Person;

/// # Summary
///
/// A wrapper around [`Person`] for use with TomlDB.
pub struct TomlPerson<'name>
(
	Person<'name>,
);
