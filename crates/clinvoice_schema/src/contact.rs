mod display;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Location;

/// # Summary
///
/// A view of [`Location`](crate::Location).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde_support", serde(untagged))]
pub enum Contact
{
	/// # Summary
	///
	/// A [`Location`](crate::Location).
	Address
	{
		location: Location,
		label: String,
		export: bool,
	},

	/// # Summary
	///
	/// An email address.
	///
	/// # Example
	///
	/// * 'foo@bar.io'
	Email
	{
		email: String,
		label: String,
		export: bool,
	},

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
	Phone
	{
		phone: String,
		export: bool,
		label: String,
	},
}

impl Contact
{
	pub fn export(&self) -> bool
	{
		match self
		{
			Self::Address {
				location: _,
				label: _,
				export,
			} |
			Self::Email {
				label: _,
				email: _,
				export,
			} |
			Self::Phone {
				label: _,
				phone: _,
				export,
			} => *export,
		}
	}

	pub fn label(&self) -> &str
	{
		match self
		{
			Contact::Address {
				location: _,
				label,
				export: _,
			} |
			Contact::Email {
				email: _,
				export: _,
				label,
			} |
			Contact::Phone {
				export: _,
				label,
				phone: _,
			} => label,
		}
	}
}
