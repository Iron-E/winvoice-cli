mod from_view;
mod hash;

use crate::{Contact, Id};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A person is a physical human being.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Person
{
	/// # Summary
	///
	/// Contact information specific to the individual [`Person`], rather than a corporation they
	/// work at.
	pub contact_info: Vec<Contact>,

	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: String,
}
