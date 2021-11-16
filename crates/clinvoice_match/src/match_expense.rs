use std::borrow::Cow;

use clinvoice_schema::{ExpenseCategory, Money};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Match, MatchStr};

/// # Summary
///
/// An [`Invoice`](clinvoice_schema::Invoice) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct MatchExpense<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub category: Match<'m, ExpenseCategory>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub cost: Match<'m, Money>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub description: MatchStr<Cow<'m, str>>,
}
