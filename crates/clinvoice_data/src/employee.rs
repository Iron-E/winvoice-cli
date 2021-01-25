use crate::{Contact, Id};
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An `Employee` is a [`Person`](super::person::Person) who completes [`Job`](super::job::Job)s
/// for an [employer](crate::Organization).
#[derive(Debug, Deserialize, Serialize)]
pub struct Employee<'email, 'phone, 'title>
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	pub contact_info: Vec<Contact<'email, 'phone>>,

	/// # Summary
	///
	/// The reference number of this [`Employee`], which can be used instead of the compound key
	/// {`organization`, `person_id`}.
	pub id: Id,

	/// # Summary
	///
	/// The reference number of the [`Organization`](crate::Organization) which this
	/// [`Employee`] is in reference to.
	pub organization_id: Id,

	/// # Summary
	///
	/// The reference number of the [`Person`](super::person::Person) which this
	/// [`Employee`] is in reference to.
	pub person_id: Id,

	/// # Summary
	///
	/// The [`Employee`]'s title  in the company.
	///
	/// # Example
	///
	/// * CEO
	/// * Media Manager
	pub title: &'title str,
}
