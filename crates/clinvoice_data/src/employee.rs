use crate::{Contact, Id};

/// # Summary
///
/// An `Employee` is a [`Person`](super::person::Person) who completes [`Job`](super::job::Job)s
/// for an [`Employer`](super::employer::Employer).
pub struct Employee<'addr, 'contact_info, 'email>
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	pub contact_info: &'contact_info [Contact<'addr, 'email>],

	/// # Summary
	///
	/// The reference number of the [`Employer`](super::employer::Employer) which this
	/// [`Employee`] is in reference to.
	pub employer_id: Id,

	/// # Summary
	///
	/// The reference number of this [`Employee`], which can be used instead of the compound key
	/// {`employer_id`, `person_id`}.
	pub id: Id,

	/// # Summary
	///
	/// The reference number of the [`Person`](super::person::Person) which this
	/// [`Employee`] is in reference to.
	pub person_id: Id,
}
