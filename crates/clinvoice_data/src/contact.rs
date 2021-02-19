use crate::Id;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A method through which something can be communicated with.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub enum Contact
{
	/// # Summary
	///
	/// A [`Location`](crate::Location).
	Address(Id),

	/// # Summary
	///
	/// An email address.
	///
	/// # Example
	///
	/// * 'foo@bar.io'
	Email(String),

	/// # Summary
	///
	/// A phone number.
	///
	/// # Example
	///
	/// * '1-603-555-1234'
	/// * '603-555-1234'
	Phone(String),
}
