use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchLocation, MatchStr};
use crate::{MatchContact, MatchSet};

/// # Summary
///
/// An [`Organization`](clinvoice_schema::Organization) with [matchable](Match) fields.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchOrganization
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub contact_info: MatchSet<MatchContact>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub location: MatchLocation,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,
}
