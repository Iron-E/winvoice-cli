use super::Money;
use crate::{Currency, ExchangeRates, Exchangeable};

impl Exchangeable for Money
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		// noop for same currency
		if self.currency == currency
		{
			return self;
		}

		let mut exchanged = self.amount / rates.index(&self.currency..&currency);
		exchanged.rescale(2);
		Self {
			amount: exchanged,
			currency,
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self::exchange(*self, currency, rates)
	}
}

#[cfg(test)]
mod tests
{
	use std::{env, fs};

	use super::{Currency, ExchangeRates, Money};
	use crate::{Error, Exchangeable, SAMPLE_EXCHANGE_RATES_CSV};

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

		let usd = Money::new(20_00, 2, Currency::Usd);

		let usd_to_jpy = usd.exchange(Currency::Jpy, &exchange_rates);
		assert_eq!(usd_to_jpy, Money::new(2195_95, 2, Currency::Jpy));

		// Assert round-trip works
		let usd_to_jpy_to_usd = usd_to_jpy.exchange(Currency::Usd, &exchange_rates);
		assert_eq!(usd, usd_to_jpy_to_usd);
	}
}
