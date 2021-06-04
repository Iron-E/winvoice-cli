mod display;

use
{
	crate::{Currency, ExchangeRates},

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
	pub fn exchange(self, currency: Currency, exchange_rates: &ExchangeRates) -> Self
	{
		// noop for same currency
		if self.currency == currency { return self; }

		let eur = self.amount / exchange_rates[self.currency];
		let mut exchanged = eur * exchange_rates[currency];
		exchanged.rescale(2);

		Self {amount: exchanged, currency}
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

#[cfg(test)]
mod tests
{
	#[test]
	fn exchange() {}
}
