use clinvoice_data::employer::Employer;

pub mod into_organization;

/// # Summary
///
/// A wrapper around [`Employer`] for use with MongoDB.
pub struct MongoEmployer
(
	Employer,
);
