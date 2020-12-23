use clinvoice_data::Location;

mod display;
mod insertable_location;
mod wrapper;

/// # Summary
///
/// A wrapper around [`Location`] for use with MongoDB.
pub struct MongoLocation<'name>
(
	Location<'name>,
);
