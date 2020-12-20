use clinvoice_data::person::Person;

/// # Summary
///
/// A wrapper around [`Person`] for use with MongoDB.
pub struct MongoPerson<'name>
(
	Person<'name>,
);
