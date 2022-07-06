mod display;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::Location;

/// The variant data of a [`Contact`]. Created because [`Contact`]s have _some_ data in common
/// (i.e. the `label`), but all of the rest is variable. Further, certain types of [`String`] data
/// (namely, [`ContactKind::Email`] and [`ContactKind::Phone`]) can be minimally verified before
/// insertion into a database which helps prevent user error.
#[cfg_attr(
	feature = "serde_support",
	derive(Deserialize, Serialize),
	serde(untagged)
)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ContactKind
{
	/// Some address, which is a [`Location`] in the real world (e.g. an office in London).
	Address(Location),

	/// An email address, e.g. `"foo@bar.io"`.
	Email(String),

	/// Any kind of information which is not covered by another [`ContactKind`] variant, for
	/// example:
	///
	/// * A username for a social media platform (e.g. [Twitter](https://www.twitter.com)) or
	///   monetary transfer service (e.g. [PayPal](https://www.paypal.com)).
	/// * A bank account number.
	/// * A crypto wallet.
	///
	/// One should not attempt to verify or constrain this data, as it is impossible to tell what it
	/// might be.
	Other(String),

	/// A phone number, with or without country code. The following should be treated as valid:
	///
	/// * '+1 603 555-1234'
	/// * '1-603-555-1234'
	/// * '16035551234'
	/// * '603 555-1234'
	/// * '603-555-1234'
	/// * '6035551234'
	Phone(String),
}

impl ContactKind
{
	/// If this is a [`ContactKind::Address`], return the inner [`Location`] value as [`Some`]. Otherwise, return [`None`].
	pub fn address(&self) -> Option<&Location>
	{
		match self
		{
			Self::Address(l) => Some(l),
			_ => None,
		}
	}

	/// If this is a [`ContactKind::Email`], return the inner [`str`] value as [`Some`]. Otherwise, return [`None`].
	pub fn email(&self) -> Option<&str>
	{
		match self
		{
			Self::Email(e) => Some(e),
			_ => None,
		}
	}

	/// If this is a [`ContactKind::Phone`], return the inner [`str`] value as [`Some`]. Otherwise, return [`None`].
	pub fn phone(&self) -> Option<&str>
	{
		match self
		{
			Self::Phone(p) => Some(p),
			_ => None,
		}
	}

	/// If this is a [`ContactKind::Other`], return the inner [`str`] value as [`Some`]. Otherwise, return [`None`].
	pub fn other(&self) -> Option<&str>
	{
		match self
		{
			Self::Other(o) => Some(o),
			_ => None,
		}
	}
}
