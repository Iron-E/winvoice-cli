mod display;

use rust_decimal::Decimal;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::{Currency, ExchangeRates};

/// # Summary
///
/// Some `amount` of `currency`.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
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
		if self.currency == currency
		{
			return self;
		}

		let eur = self.amount / exchange_rates[self.currency];
		let mut exchanged = eur * exchange_rates[currency];
		exchanged.rescale(2);

		Self {
			amount: exchanged,
			currency,
		}
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
		Self {
			amount: Decimal::new(amount, decimal_places),
			currency,
		}
	}
}

#[cfg(test)]
mod tests
{
	use std::{env, fs};

	use super::{Currency, ExchangeRates, Money};
	use crate::{Error, SAMPLE_EXCHANGE_RATES_CSV};

	#[test]
	fn exchange()
	{
		let filepath = env::temp_dir()
			.join("clinvoice_finance")
			.join("money")
			.join("exchange.csv");

		if filepath.is_file()
		{
			fs::remove_file(&filepath).unwrap();
		}

		assert!(fs::read_to_string(&filepath)
			.map_err(Error::from)
			.and_then(|s| s.parse::<ExchangeRates>())
			.is_err());

		let parent = filepath.parent().unwrap();
		if !parent.is_dir()
		{
			fs::create_dir_all(parent).unwrap();
		}

		fs::write(&filepath, SAMPLE_EXCHANGE_RATES_CSV).unwrap();

		assert!(filepath.is_file());

		let exchange_rates = fs::read_to_string(&filepath)
			.map_err(Error::from)
			.and_then(|s| s.parse::<ExchangeRates>())
			.unwrap();

		let usd = Money::new(20_00, 2, Currency::USD);

		let usd_to_jpy = usd.exchange(Currency::JPY, &exchange_rates);
		assert_eq!(usd_to_jpy, Money::new(2195_95, 2, Currency::JPY));

		// Assert round-trip works
		let usd_to_jpy_to_usd = usd_to_jpy.exchange(Currency::USD, &exchange_rates);
		assert_eq!(usd, usd_to_jpy_to_usd);
	}
}
