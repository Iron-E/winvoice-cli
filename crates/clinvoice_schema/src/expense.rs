mod display;
mod exchangeable;
mod restorable_serde;

use clinvoice_finance::Money;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Id;

/// # Summary
///
/// A representation of some item or service which a [client](super::Organization)'s money was
/// spent to acquire for a [`Job`](super::Job) on a [`Timesheet`](super::Timesheet).
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Expense
{
	/// # Summary
	///
	/// The [`Id`] of this [`Expense`].
	pub id: Id,

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

	/// # Summary
	///
	/// The [`Timesheet`] that this [`Expense`] is attached to.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub timesheet_id: Id,
}
