mod outer_location;

use clinvoice_data::Id;
pub use outer_location::OuterLocation;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Location`](clinvoice_data::Location) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Location<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub outer: OuterLocation<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}
