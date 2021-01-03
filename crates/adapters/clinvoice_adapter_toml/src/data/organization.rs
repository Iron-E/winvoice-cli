use clinvoice_data::Organization;

pub mod into_location;

/// # Summary
///
/// A wrapper around [`Organization`] for use with TomlDB.
pub struct TomlOrganization<'name, 'rep_title>
(
	Organization<'name, 'rep_title>,
);
