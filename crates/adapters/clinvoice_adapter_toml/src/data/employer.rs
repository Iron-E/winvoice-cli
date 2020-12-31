use clinvoice_data::Employer;

mod into_organization;

/// # Summary
///
/// A wrapper around [`Employer`] for use with TomlDB.
pub struct TomlEmployer
(
	Employer,
);
