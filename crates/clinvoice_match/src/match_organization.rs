use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchLocation, MatchStr};

/// # Summary
///
/// An [`Organization`](clinvoice_schema::Organization) with [matchable](Match) fields.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchOrganization
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub location: MatchLocation,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}
