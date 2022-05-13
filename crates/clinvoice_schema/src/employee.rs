mod display;
mod hash;
mod partial_eq;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Contact, Organization, Person};
use crate::Id;

/// # Summary
///
/// A view of [`Employee`](crate::Employee).
#[derive(Clone, Debug, Default, Eq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Employee
{
	/// # Summary
	///
	/// Contact information specific to the [`Organization`] that the [`Employee`] does work for.
	pub contact_info: Vec<Contact>,

	/// # Summary
	///
	/// The reference number of this [`Employee`], which can be used instead of the compound key
	/// {`organization`, `person_id`}.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The reference number of the [`Organization`](crate::Organization) which this
	/// [`Employee`] is in reference to.
	pub organization: Organization,

	/// # Summary
	///
	/// The reference number of the [`Person`](super::person::Person) which this
	/// [`Employee`] is in reference to.
	pub person: Person,

	/// # Summary
	///
	/// The status of the employee.
	///
	/// # Remarks
	///
	/// Setting this field to "Not employed", or "ex-employee" is a viable alternative to deletion.
	pub status: String,

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
