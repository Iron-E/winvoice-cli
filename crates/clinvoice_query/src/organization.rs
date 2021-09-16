use clinvoice_data::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Location, Match, MatchStr};

/// # Summary
///
/// An [`Organization`](clinvoice_data::Organization) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Organization<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<'m, Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub location: Location<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}
