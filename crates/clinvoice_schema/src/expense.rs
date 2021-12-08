mod display;

use clinvoice_finance::Money;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A representation of some item or service which a [client](crate::Organization)'s money was
/// spent to acquire for a [`Job`](crate::Job) on a [`Timesheet`](crate::Timesheet).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Expense
{
	/// # Summary
	///
	/// What kind of [`Expense`] this is.
	pub category: String,

	/// # Summary
	///
	/// The amount of [`Money`] that this [`Expense`] cost.
	pub cost: Money,

	/// # Summary
	///
	/// A description of what this [`Expense`] is.
	pub description: String,
}
