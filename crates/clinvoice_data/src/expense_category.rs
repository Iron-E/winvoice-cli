#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Common categories of [`Expense`](crate::Expense)s.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub enum ExpenseCategory
{
	Food,
	Hosting,
	Item,
	Other,
	Software,
	Travel,
}
