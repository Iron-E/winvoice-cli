mod display;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// The status of an [`Employee`](crate::Employee)
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub enum EmployeeStatus
{
	/// # Summary
	///
	/// The [`Employee`](crate::Employee) is employed at the
	/// [`Organization`](crate::Organization).
	Employed,

	/// # Summary
	///
	/// The [`Employee`](crate::Employee) is not employed at the
	/// [`Organization`](crate::Organization).
	NotEmployed,

	/// # Summary
	///
	/// The [`Employee`](crate::Employee) is a representative
	/// of the [`Organization`](crate::Organization).
	Representative,
}
