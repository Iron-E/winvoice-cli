use
{
	std::{collections::HashMap, convert::TryFrom, fs, io, path::Path},

	super::ExchangeRates,
	crate::{Currency, UnsupportedCurrencyError},

	rust_decimal::{Decimal, Error as DecimalError},
};

impl TryFrom<&Path> for ExchangeRates
{
	type Error = io::Error;

	fn try_from(path: &Path) -> io::Result<Self>
	{
		let contents = fs::read_to_string(path)?;
		let (currencies, rates) =
		{
			let mut csv = contents.split('\n').map(|line| line.split(", "));
			(csv.next().unwrap(), csv.next().unwrap())
		};

		let mut map: HashMap<_, _> = currencies.zip(rates).skip(1).filter(|(c, _)| !c.is_empty()).map(|(c, r)|
		(
			c.parse::<Currency>().unwrap_or_else(|_| panic!("{}", UnsupportedCurrencyError(c.into()))),
			r.parse::<Decimal>().unwrap_or_else(|_| panic!("{}", DecimalError::ErrorString(r.into()))),
		)).collect();

		map.insert(Currency::EUR, 1.into());

		Ok(ExchangeRates(map))
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{env, time::Instant},

		super::{Currency, Decimal, ExchangeRates, fs, TryFrom},
	};

	#[test]
	fn try_from()
	{
		let filepath = env::temp_dir().join("clinvoice_finance").join("exchange-rates").join("try-from.csv");

		if filepath.is_file() { fs::remove_file(&filepath).unwrap(); }

		assert!(ExchangeRates::try_from(filepath.as_path()).is_err());

		let parent = filepath.parent().unwrap();
		if !parent.is_dir() { fs::create_dir_all(parent).unwrap(); }

		fs::write(&filepath, "Date, USD, JPY, BGN, CZK, DKK, GBP, HUF, PLN, RON, SEK, CHF, ISK, NOK, HRK, RUB, TRY, AUD, BRL, CAD, CNY, HKD, IDR, ILS, INR, KRW, MXN, MYR, NZD, PHP, SGD, THB, ZAR, \n03 June 2021, 1.2187, 133.81, 1.9558, 25.448, 7.4365, 0.85955, 345.82, 4.4520, 4.9220, 10.1145, 1.0961, 146.30, 10.1501, 7.5013, 89.2163, 10.5650, 1.5792, 6.1894, 1.4710, 7.7910, 9.4551, 17420.91, 3.9598, 88.8755, 1357.75, 24.3300, 5.0241, 1.6915, 58.208, 1.6141, 37.938, 16.5218, ").unwrap();

		assert!(filepath.is_file());

		let start = Instant::now();
		let rates = ExchangeRates::try_from(filepath.as_path()).unwrap();
		println!("\n>>>>> ExchangeRates::try_from {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		assert_eq!(rates[Currency::AUD], Decimal::new(15792, 4));
		assert_eq!(rates[Currency::BGN], Decimal::new(19558, 4));
		assert_eq!(rates[Currency::BRL], Decimal::new(61894, 4));
		assert_eq!(rates[Currency::CAD], Decimal::new(14710, 4));
		assert_eq!(rates[Currency::CHF], Decimal::new(10961, 4));
		assert_eq!(rates[Currency::CNY], Decimal::new(77910, 4));
		assert_eq!(rates[Currency::CZK], Decimal::new(25448, 3));
		assert_eq!(rates[Currency::DKK], Decimal::new(74365, 4));
		assert_eq!(rates[Currency::EUR], Decimal::new(1, 0));
		assert_eq!(rates[Currency::GBP], Decimal::new(85955, 5));
		assert_eq!(rates[Currency::HKD], Decimal::new(94551, 4));
		assert_eq!(rates[Currency::HRK], Decimal::new(75013, 4));
		assert_eq!(rates[Currency::HUF], Decimal::new(34582, 2));
		assert_eq!(rates[Currency::IDR], Decimal::new(1742091, 2));
		assert_eq!(rates[Currency::ILS], Decimal::new(39598, 4));
		assert_eq!(rates[Currency::INR], Decimal::new(888755, 4));
		assert_eq!(rates[Currency::ISK], Decimal::new(14630, 2));
		assert_eq!(rates[Currency::JPY], Decimal::new(13381, 2));
		assert_eq!(rates[Currency::KRW], Decimal::new(135775, 2));
		assert_eq!(rates[Currency::MXN], Decimal::new(243300, 4));
		assert_eq!(rates[Currency::MYR], Decimal::new(50241, 4));
		assert_eq!(rates[Currency::NOK], Decimal::new(101501, 4));
		assert_eq!(rates[Currency::NZD], Decimal::new(16915, 4));
		assert_eq!(rates[Currency::PHP], Decimal::new(58208, 3));
		assert_eq!(rates[Currency::PLN], Decimal::new(44520, 4));
		assert_eq!(rates[Currency::RON], Decimal::new(49220, 4));
		assert_eq!(rates[Currency::RUB], Decimal::new(892163, 4));
		assert_eq!(rates[Currency::SEK], Decimal::new(101145, 4));
		assert_eq!(rates[Currency::SGD], Decimal::new(16141, 4));
		assert_eq!(rates[Currency::THB], Decimal::new(37938, 3));
		assert_eq!(rates[Currency::TRY], Decimal::new(105650, 4));
		assert_eq!(rates[Currency::USD], Decimal::new(12187, 4));
		assert_eq!(rates[Currency::ZAR], Decimal::new(165218, 4));
	}
}
