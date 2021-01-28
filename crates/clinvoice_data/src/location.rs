use uuid::Uuid;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A physical space where other `Location`s or
/// [`Organization`](super::organization::Organization)s exist.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Location<'name>
{
	/// # Summary
	///
	/// The reference number of the [`Location`].
	pub id: Uuid,

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
	pub outer_id: Option<Uuid>,

	/// # Summary
	///
	/// The name of the [`Location`].
	pub name: &'name str,
}
