mod display;
mod insertable_location;
mod wrapper;

use clinvoice_data::Location;

/// # Summary
///
/// A wrapper around [`Location`] for use with TomlDB.
pub struct TomlLocation<'name>
(
	Location<'name>,
);
