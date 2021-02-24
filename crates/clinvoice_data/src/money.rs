mod display;

use rust_decimal::Decimal;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Some `amount` of `currency`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Money
{
	/// # Summary
	///
	/// The amount of `currency` that this [`Money`] represents.
	pub amount: Decimal,

	/// # Summary
	///
	/// The `currency` that this [`Money`] is in.
	pub currency: String,
}

impl Money
{
	/// # Summary
	///
	/// Create a new [`Money`] struct.
	///
	/// # Paramters
	///
	/// See [`Money`]'s fields.
	///
	/// # Returns
	///
	/// A new [`Money`].
	pub fn new(amount: Decimal, currency: &str) -> Self
	{
		return Self {amount, currency: currency.into()};
	}
}
