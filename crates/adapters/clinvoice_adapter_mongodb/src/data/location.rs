use clinvoice_data::Location;

pub mod display;
pub mod insertable_location;
pub mod wrapper;

/// # Summary
///
/// A wrapper around [`Location`] for use with MongoDB.
pub struct MongoLocation<'name>
(
	Location<'name>,
);
