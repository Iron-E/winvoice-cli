use core::ops::Index;

use rust_decimal::Decimal;

use super::ExchangeRates;
use crate::Currency;

impl Index<Currency> for ExchangeRates
{
	type Output = Decimal;

	fn index(&self, index: Currency) -> &Self::Output
	{
		self
			.get(index)
			.unwrap_or_else(|| panic!("{} was not found in this set of exchange rates", index))
	}
}
