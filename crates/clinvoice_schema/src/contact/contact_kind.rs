mod display;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::Location;

/// # Summary
///
/// The specific kind of [`Contact`] that something is.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde_support", serde(untagged))]
pub enum ContactKind
{
	/// # Summary
	///
	/// A [`Location`](crate::Location).
	Address(Location),

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
	/// The following are valid for numbers with country code:
	///
	/// * '+1 (603) 555-1234'
	/// * '1-603-555-1234'
	/// * '16035551234'
	///
	/// The following are valid for numbers without country code:
	///
	/// * '(603) 555-1234'
	/// * '603-555-1234'
	/// * '6035551234'
	Phone(String),
}
