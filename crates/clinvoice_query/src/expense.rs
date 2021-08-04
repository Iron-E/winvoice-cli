use clinvoice_data::{
	finance::Money,
	ExpenseCategory,
};
#[cfg(feature = "serde_support")]
use serde::{
	Deserialize,
	Serialize,
};

use super::{
	Match,
	MatchStr,
	Result,
};

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

impl Expense<'_>
{
	/// # Summary
	///
	/// Return `true` if `invoice` is a match.
	pub fn set_matches<'item>(
		&self,
		expenses: &mut impl Iterator<Item = &'item clinvoice_data::Expense>,
	) -> Result<bool>
	{
		Ok(self
			.category
			.set_matches(&expenses.by_ref().map(|e| &e.category).collect()) &&
			self
				.cost
				.set_matches(&expenses.by_ref().map(|e| &e.cost).collect()) &&
			self
				.description
				.set_matches(&mut expenses.map(|e| e.description.as_ref()))?)
	}
}
