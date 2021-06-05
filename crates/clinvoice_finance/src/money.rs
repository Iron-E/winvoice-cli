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
	use
	{
		std::{convert::TryFrom, env, fs, time::Instant},

		super::{Currency, ExchangeRates, Money},
	};

	#[test]
	fn exchange()
	{
		let filepath = env::temp_dir().join("clinvoice_finance").join("money").join("exchange.csv");

		if filepath.is_file() { fs::remove_file(&filepath).unwrap(); }

		assert!(ExchangeRates::try_from(filepath.as_path()).is_err());

		let parent = filepath.parent().unwrap();
		if !parent.is_dir() { fs::create_dir_all(parent).unwrap(); }

		fs::write(&filepath, "Date, USD, JPY, BGN, CZK, DKK, GBP, HUF, PLN, RON, SEK, CHF, ISK, NOK, HRK, RUB, TRY, AUD, BRL, CAD, CNY, HKD, IDR, ILS, INR, KRW, MXN, MYR, NZD, PHP, SGD, THB, ZAR, \n03 June 2021, 1.2187, 133.81, 1.9558, 25.448, 7.4365, 0.85955, 345.82, 4.4520, 4.9220, 10.1145, 1.0961, 146.30, 10.1501, 7.5013, 89.2163, 10.5650, 1.5792, 6.1894, 1.4710, 7.7910, 9.4551, 17420.91, 3.9598, 88.8755, 1357.75, 24.3300, 5.0241, 1.6915, 58.208, 1.6141, 37.938, 16.5218, ").unwrap();

		assert!(filepath.is_file());

		let exchange_rates = ExchangeRates::try_from(filepath.as_path()).unwrap();

		let usd = Money::new(2000, 2, Currency::USD);

		let start = Instant::now();
		let usd_to_jpy = usd.exchange(Currency::JPY, &exchange_rates);
		assert_eq!(usd_to_jpy, Money::new(2195_95, 2, Currency::JPY));

		// Assert round-trip works
		let usd_to_jpy_to_usd = usd_to_jpy.exchange(Currency::USD, &exchange_rates);
		assert_eq!(usd, usd_to_jpy_to_usd);
		println!("\n>>>>> Money::exchange {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 2);
	}
}
