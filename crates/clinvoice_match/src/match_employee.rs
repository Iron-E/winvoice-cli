use clinvoice_schema::Id;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Employee`](clinvoice_schema::Employee) with [matchable](Match) fields.
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MatchEmployee
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub id: Match<Id>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub name: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub status: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub title: MatchStr<String>,
}
