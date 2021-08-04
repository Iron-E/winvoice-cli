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
/// A physical space where other `Location`s or
/// [`Organization`](super::organization::Organization)s exist.
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Location
{
	/// # Summary
	///
	/// The reference number of the [`Location`].
	pub id: Id,

	/// # Summary
	///
	/// The reference number of the [`Location`] in which _this_ [`Location`] resides.
	///
	/// # Remarks
	///
	/// * If there is [`Some`] `outer_id`, it means that `this` [`Location`] is located inside
	///   another.
	/// * If there is [`None`] `outer_id`, it means that `this` [`Location`] is an outermost
	///   residence.
	pub outer_id: Option<Id>,

	/// # Summary
	///
	/// The name of the [`Location`].
	pub name: String,
}
