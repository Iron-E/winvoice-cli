use std::{collections::HashMap, str::FromStr};

use rust_decimal::Decimal;

use super::ExchangeRates;
use crate::{Currency, Error, Result};

impl FromStr for ExchangeRates
{
	type Err = Error;

	fn from_str(csv: &str) -> Result<Self>
	{
		let (currencies, rates) = {
			let mut grid = csv.split('\n').map(|line| line.split(", "));
			(
				grid
					.next()
					.expect("There should be a currency column in this CSV"),
				grid
					.next()
					.expect("There should be an exchange rate column in this CSV"),
			)
		};

		let mut map = HashMap::with_capacity(Currency::count());
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
