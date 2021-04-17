use
{
	crate::data::Match,

	clinvoice_data::{ExpenseCategory, Money},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An [`Invoice`](clinvoice_data::Invoice) with [matchable](Match) fields.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Expense<'m>
{
	#[cfg_attr(feature="serde_support", serde(default))]
	pub category: Match<'m, ExpenseCategory>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub cost: Match<'m, Money>,

	#[cfg_attr(feature="serde_support", serde(default))]
	pub description: Match<'m, String>,
}

impl Expense<'_>
{
	/// # Summary
	///
	/// Return `true` if `invoice` is a match.
	pub fn set_matches<'item>(&self, mut expenses: impl Iterator<Item=&'item clinvoice_data::Expense>) -> bool
	{
		self.category.set_matches(&expenses.by_ref().map(|e| &e.category).collect()) &&
		self.cost.set_matches(&expenses.by_ref().map(|e| &e.cost).collect()) &&
		self.description.set_matches(&expenses.map(|e| &e.description).collect())
	}
}
