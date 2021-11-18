use std::{collections::HashMap, convert::TryFrom, fs, path::Path};

use rust_decimal::Decimal;

use super::ExchangeRates;
use crate::{Currency, Error, Result};

impl TryFrom<&Path> for ExchangeRates
{
	type Error = Error;

	fn try_from(path: &Path) -> Result<Self>
	{
		let contents = fs::read_to_string(path)?;
		let (currencies, rates) = {
			let mut csv = contents.split('\n').map(|line| line.split(", "));
			(
				csv.next()
					.expect("There should be a column of currencies in this CSV"),
				csv.next()
					.expect("There should be a column of exchange rates in this CSV"),
			)
		};

		let mut exchange_rates = ExchangeRates(HashMap::new());
		exchange_rates.0.insert(Currency::EUR, 1.into());

		currencies
			.zip(rates)
			.skip(1)
			.filter(|(c, _)| !c.is_empty())
			.try_for_each(|(c, r)| -> Result<()> {
				let currency = c.parse::<Currency>()?;
				exchange_rates.0.insert(currency, r.parse::<Decimal>()?);
				Ok(())
			})
			.and(Ok(exchange_rates))
	}
}

#[cfg(test)]
mod tests
{
	use std::env;

	use super::{fs, Currency, Decimal, ExchangeRates, TryFrom};

	#[test]
	fn try_from()
	{
		let filepath = env::temp_dir()
			.join("clinvoice_finance")
			.join("exchange-rates")
			.join("try-from.csv");

		if filepath.is_file()
		{
			fs::remove_file(&filepath).unwrap();
		}

		assert!(ExchangeRates::try_from(filepath.as_path()).is_err());

		let parent = filepath.parent().unwrap();
		if !parent.is_dir()
		{
			fs::create_dir_all(parent).unwrap();
		}

		fs::write(
			&filepath,
			"Date, USD, JPY, BGN, CZK, DKK, GBP, HUF, PLN, RON, SEK, CHF, ISK, NOK, HRK, RUB, TRY, \
			 AUD, BRL, CAD, CNY, HKD, IDR, ILS, INR, KRW, MXN, MYR, NZD, PHP, SGD, THB, ZAR, \n03 \
			 June 2021, 1.2187, 133.81, 1.9558, 25.448, 7.4365, 0.85955, 345.82, 4.4520, 4.9220, \
			 10.1145, 1.0961, 146.30, 10.1501, 7.5013, 89.2163, 10.5650, 1.5792, 6.1894, 1.4710, \
			 7.7910, 9.4551, 17420.91, 3.9598, 88.8755, 1357.75, 24.3300, 5.0241, 1.6915, 58.208, \
			 1.6141, 37.938, 16.5218, ",
		)
		.unwrap();

		assert!(filepath.is_file());

		let exchange_rates = ExchangeRates::try_from(filepath.as_path()).unwrap();

		assert_eq!(exchange_rates[Currency::AUD], Decimal::new(1_5792, 4));
		assert_eq!(exchange_rates[Currency::BGN], Decimal::new(1_9558, 4));
		assert_eq!(exchange_rates[Currency::BRL], Decimal::new(6_1894, 4));
		assert_eq!(exchange_rates[Currency::CAD], Decimal::new(1_4710, 4));
		assert_eq!(exchange_rates[Currency::CHF], Decimal::new(1_0961, 4));
		assert_eq!(exchange_rates[Currency::CNY], Decimal::new(7_7910, 4));
		assert_eq!(exchange_rates[Currency::CZK], Decimal::new(25_448, 3));
		assert_eq!(exchange_rates[Currency::DKK], Decimal::new(7_4365, 4));
		assert_eq!(exchange_rates[Currency::EUR], Decimal::new(1, 0));
		assert_eq!(exchange_rates[Currency::GBP], Decimal::new(85955, 5));
		assert_eq!(exchange_rates[Currency::HKD], Decimal::new(9_4551, 4));
		assert_eq!(exchange_rates[Currency::HRK], Decimal::new(7_5013, 4));
		assert_eq!(exchange_rates[Currency::HUF], Decimal::new(345_82, 2));
		assert_eq!(exchange_rates[Currency::IDR], Decimal::new(17420_91, 2));
		assert_eq!(exchange_rates[Currency::ILS], Decimal::new(3_9598, 4));
		assert_eq!(exchange_rates[Currency::INR], Decimal::new(8_88755, 4));
		assert_eq!(exchange_rates[Currency::ISK], Decimal::new(146_30, 2));
		assert_eq!(exchange_rates[Currency::JPY], Decimal::new(133_81, 2));
		assert_eq!(exchange_rates[Currency::KRW], Decimal::new(1357_75, 2));
		assert_eq!(exchange_rates[Currency::MXN], Decimal::new(24_3300, 4));
		assert_eq!(exchange_rates[Currency::MYR], Decimal::new(5_0241, 4));
		assert_eq!(exchange_rates[Currency::NOK], Decimal::new(10_1501, 4));
		assert_eq!(exchange_rates[Currency::NZD], Decimal::new(1_6915, 4));
		assert_eq!(exchange_rates[Currency::PHP], Decimal::new(58_208, 3));
		assert_eq!(exchange_rates[Currency::PLN], Decimal::new(4_4520, 4));
		assert_eq!(exchange_rates[Currency::RON], Decimal::new(4_9220, 4));
		assert_eq!(exchange_rates[Currency::RUB], Decimal::new(89_2163, 4));
		assert_eq!(exchange_rates[Currency::SEK], Decimal::new(10_1145, 4));
		assert_eq!(exchange_rates[Currency::SGD], Decimal::new(1_6141, 4));
		assert_eq!(exchange_rates[Currency::THB], Decimal::new(37_938, 3));
		assert_eq!(exchange_rates[Currency::TRY], Decimal::new(10_5650, 4));
		assert_eq!(exchange_rates[Currency::USD], Decimal::new(1_2187, 4));
		assert_eq!(exchange_rates[Currency::ZAR], Decimal::new(16_5218, 4));
	}
}
