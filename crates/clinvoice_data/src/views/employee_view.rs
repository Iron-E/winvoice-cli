mod display;
mod hash;

use
{
	crate::EmployeeStatus,
	super::{ContactView, OrganizationView, PersonView},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A view of [`Employee`](crate::Employee).
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct EmployeeView
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	pub contact_info: Vec<ContactView>,

	/// # Summary
	///
	/// The reference number of the [`Organization`](crate::Organization) which this
	/// [`Employee`] is in reference to.
	pub organization: OrganizationView,

	/// # Summary
	///
	/// The reference number of the [`Person`](super::person::Person) which this
	/// [`Employee`] is in reference to.
	pub person: PersonView,

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
