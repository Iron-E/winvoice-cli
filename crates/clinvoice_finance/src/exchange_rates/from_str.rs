use std::{collections::HashMap, str::FromStr};

use rust_decimal::Decimal;
use strum::EnumCount;

use super::ExchangeRates;
use crate::{Currency, Error, Result};

impl FromStr for ExchangeRates
{
	type Err = Error;

	fn from_str(csv: &str) -> Result<Self>
	{
		let (currencies, rates) = {
			let mut columns_by_values = csv.split('\n').map(|line| line.split(", "));
			(
				columns_by_values
					.next()
					.ok_or_else(|| Error::EcbCsvDecode("there was no currency column".into()))?,
				columns_by_values
					.next()
					.ok_or_else(|| Error::EcbCsvDecode("there was no exchange rate column".into()))?,
			)
		};

		let mut map = HashMap::with_capacity(Currency::COUNT);
		map.insert(Default::default(), 1.into());

		currencies
			.zip(rates)
			.skip(1)
			.filter(|(c, _)| !c.is_empty())
			.try_for_each(|(c, r)| -> Result<()> {
				let currency = c.parse::<Currency>()?;
				map.insert(currency, r.parse::<Decimal>()?);
				Ok(())
			})
			.and(Ok(ExchangeRates(map)))
	}
}
