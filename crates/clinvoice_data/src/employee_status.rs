mod display;
mod from_str;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// The status of an [`Employee`](crate::Employee)
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
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

impl EmployeeStatus
{
	/// # Summary
	///
	/// Convert this [`EmployeeStatus`] into a `&'static str` for purposes of either [`Display`] or
	/// comparison without allocation.
	pub const fn as_str(&self) -> &'static str
	{
		match self
		{
			Self::Employed => "Employed",
			Self::NotEmployed => "Not employed",
			Self::Representative => "Representative",
		}
	}
}
