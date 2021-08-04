mod from_view;
mod hash;
mod partial_eq;

#[cfg(feature = "serde_support")]
use serde::{
	Deserialize,
	Serialize,
};

use crate::Id;

/// # Summary
///
/// A person is a physical human being.
#[derive(Clone, Debug, Eq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Person
{
	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: String,
}
