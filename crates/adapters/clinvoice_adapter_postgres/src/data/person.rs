use clinvoice_data::Person;

/// # Summary
///
/// A wrapper around [`Person`] for use with MongoDB.
pub struct MongoPerson<'name>
(
	Person<'name>,
);
