mod display;
mod restorable_serde;

use clinvoice_finance::Money;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::Id;

/// # Summary
///
/// A representation of some item or service which a [client](crate::Organization)'s money was
/// spent to acquire for a [`Job`](crate::Job) on a [`Timesheet`](crate::Timesheet).
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
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
