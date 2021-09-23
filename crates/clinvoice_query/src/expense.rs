use clinvoice_data::{ExpenseCategory, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Expense<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub category: Match<'m, ExpenseCategory>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub cost: Match<'m, Money>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub description: MatchStr<String>,
}
