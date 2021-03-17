mod from_view;
mod hash;
mod partial_eq;

use
{
	crate::{Contact, Id},
	std::collections::HashMap,
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A person is a physical human being.
#[derive(Clone, Debug, Eq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Person
{
	/// # Summary
	///
	/// Contact information specific to the individual [`Person`], rather than a corporation they
	/// work at.
	///
	/// # Remarks
	///
	/// Keys in the [map](HashMap) are labels of the contact is (e.g. "Primary Phone").
	pub contact_info: HashMap<String, Contact>,

	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: String,
}
