mod display;
mod restorable_serde;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Location;
use crate::Id;

/// # Summary
///
/// An `Organization` is a facilitator of business.
///
/// # Remarks
///
/// An `Organization` can be a person, or an entire business. If one is self-employed, then the
/// `Organization` would simply be themselves.
///
/// An `Organization` has no specific affitilation to the user, and as such can be both a
/// Client and an Emlpoyer at the same time.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Organization
{
	/// # Summary
	///
	/// The unique reference number for this [`Organization`].
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The reference umber of the [`Location`](super::location::Location) where this
	/// [`Organization`] resides.
	pub location: Location,

	/// # Summary
	///
	/// The name of the [`Organization`].
	pub name: String,
}
