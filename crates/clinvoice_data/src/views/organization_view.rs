use super::LocationView;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct OrganizationView
{
	/// # Summary
	///
	/// The reference umber of the [`Location`](super::location::Location) where this
	/// [`Organization`] resides.
	pub location: LocationView,

	/// # Summary
	///
	/// The name of the [`Organization`].
	pub name: String,
}
