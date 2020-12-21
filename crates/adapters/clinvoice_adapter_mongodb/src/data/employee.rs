use clinvoice_data::employee::Employee;

pub mod into_employer;
pub mod into_person;

/// # Summary
///
/// Wrapper around [`Employee`] for use with MongoDB.
pub struct MongoEmployee
(
	Employee,
);
