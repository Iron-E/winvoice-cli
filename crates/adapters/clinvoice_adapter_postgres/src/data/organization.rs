use clinvoice_data::Organization;

pub mod into_location;

/// # Summary
///
/// A wrapper around [`Organization`] for use with MongoDB.
pub struct MongoOrganization<'name>
(
	Organization<'name>,
);
