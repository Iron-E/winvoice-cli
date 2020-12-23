use clinvoice_data::Employee;

mod into_employer;
mod into_person;

/// # Summary
///
/// Wrapper around [`Employee`] for use with MongoDB.
pub struct MongoEmployee
(
	Employee,
);
