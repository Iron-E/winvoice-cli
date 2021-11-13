mod match_outer_location;

use clinvoice_schema::Id;
pub use match_outer_location::MatchOuterLocation;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Location`](clinvoice_schema::Location) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchLocation<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub outer: MatchOuterLocation<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}
