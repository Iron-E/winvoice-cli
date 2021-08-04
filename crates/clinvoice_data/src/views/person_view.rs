mod display;
mod from_person;
mod hash;
mod partial_eq;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{
	Deserialize,
	Serialize,
};

use crate::Id;

/// # Summary
//
/// A view of [`Person`](crate::Person).
#[derive(Clone, Debug, Eq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct PersonView
{
	/// # Summary
	///
	/// This is the unique reference number for the [`Person`].
	///
	/// # Remarks
	///
	/// The other `View` structures do not contain an `id` field because they have enough
	/// information for unique identification and hashing. However, the [`Person`] requires this
	/// field in order to be uniquely identified.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// This is the name of the [`Person`].
	pub name: String,
}
