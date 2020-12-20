use clinvoice_data::location::Location;

pub mod display;
pub mod implementation;

/// # Summary
///
/// A wrapper around [`Location`] for use with MongoDB.
pub struct MongoLocation<'name>
(
	Location<'name>,
);
