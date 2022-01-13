use clinvoice_schema::Money;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Invoice`](clinvoice_schema::Invoice) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchExpense
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub category: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub cost: Match<Money>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub description: MatchStr<String>,
}
