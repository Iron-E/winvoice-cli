mod display;

#[cfg(feature = "serde_support")]
use serde::{
	Deserialize,
	Serialize,
};

/// # Summary
///
/// Common categories of [`Expense`](crate::Expense)s.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum ExpenseCategory
{
	/// # Summary
	///
	/// A food item, such as a hamburger or rice. Also covers drinks.
	Food,

	/// # Summary
	///
	/// A physical good, such as a pen or a bundle of paper.
	Item,

	/// # Summary
	///
	/// Anything not covered by another category.
	Other,

	/// # Summary
	///
	/// A payment for a service, such as internet hosting or cleaning.
	Service,

	/// # Summary
	///
	/// A digital good, such as Microsoft Office.
	Software,

	/// # Summary
	///
	/// Cost of travel, such as gas or car maintenance.
	Travel,
}
