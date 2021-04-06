mod from_view;
mod hash;
mod partial_eq;

use
{
	std::collections::HashMap,

	crate::{Contact, EmployeeStatus, Id},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An `Employee` is a [`Person`](super::person::Person) who completes [`Job`](super::job::Job)s
/// for an [employer](crate::Organization).
#[derive(Clone, Debug, Eq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Employee
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	///
	/// # Remarks
	///
	/// Keys in the [map](HashMap) are labels of the contact is (e.g. "Primary Phone").
	pub contact_info: HashMap<String, Contact>,

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
	/// The status of the employee.
	///
	/// # Remarks
	///
	/// Flagging this field as [`NotEmployed`](EmployeeStatus::NOT_EMPLOYED) is a viable alternative to deletion.
	pub status: EmployeeStatus,

	/// # Summary
	///
	/// The [`Employee`]'s title  in the company.
	///
	/// # Example
	///
	/// * CEO
	/// * Media Manager
	pub title: String,
}
