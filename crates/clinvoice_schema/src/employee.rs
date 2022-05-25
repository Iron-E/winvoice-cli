mod display;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Organization;
use crate::Id;

/// # Summary
///
/// A view of [`Employee`](crate::Employee).
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Employee
{
	/// # Summary
	///
	/// The reference number of this [`Employee`], which is unique to each [`employee`].
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The name of the [`Employee`].
	pub name: String,

	/// # Summary
	///
	/// The reference number of the [`Organization`](crate::Organization) which this
	/// [`Employee`] is in reference to.
	pub organization: Organization,

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
