use super::id::Id;

pub mod into_employer;
pub mod into_person;

/// # Summary
///
/// An `Employee` is a [`Person`](super::person::Person) who completes [`Job`](super::job::Job)s
/// for an [`Employer`](super::employer::Employer).
pub struct Employee
{
	/// # Summary
	///
	/// The reference number of the [`Employer`](super::employer::Employer) which this
	/// [`Employee`] is in reference to.
	_employer_id: Id,

	/// # Summary
	///
	/// The reference number of the [`Person`](super::person::Person) which this
	/// [`Employee`] is in reference to.
	_person_id: Id,
}
