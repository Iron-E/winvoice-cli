mod display;

use
{
	crate::Currency,

	rust_decimal::Decimal,
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Some `amount` of `currency`.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
	pub currency: Currency,
}

impl Money
{
	/// # Summary
	///
	/// Exchange some [`Money`] into another `currency`.
	pub fn exchange(self, currency: Currency) -> Self
	{
		// noop for same currency
		if self.currency == currency { return self; }

		todo!("1. `GET` ECB data\n2. Convert current currency to EUR\n3. Convert EUR to desired currency")
	}

	/// # Summary
	///
	/// Create a new [`Money`] struct.
	///
	/// # Paramters
	///
	/// * `amount`, the amount of [`Money`] __without decimals__ (e.g. '$30.00' => 3000).
	/// * `currency`, the ISO currency code which this `amount` is represented in.
	/// * `scale`, the number of decimal places (e.g. '$30.00' => 2).
	///
	/// # Returns
	///
	/// A new [`Money`].
	pub fn new(amount: i64, decimal_places: u32, currency: Currency) -> Self
	{
		Self {amount: Decimal::new(amount, decimal_places), currency}
	}
}
