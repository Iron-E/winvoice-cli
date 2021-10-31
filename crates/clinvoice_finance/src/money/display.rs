use core::fmt::{Display, Formatter, Result};

use super::Money;

impl Display for Money
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} {}", self.amount, self.currency)
	}
}

#[cfg(test)]
mod tests
{
	use super::Money;
	use crate::Currency;

	#[test]
	fn display()
	{
		assert_eq!(
			format!("{}", Money::new(50_00, 2, Currency::USD)),
			"50.00 USD"
		);
		assert_eq!(
			format!("{}", Money::new(90_00, 2, Currency::EUR)),
			"90.00 EUR"
		);
		assert_eq!(
			format!("{}", Money::new(20000, 0, Currency::JPY)),
			"20000 JPY"
		);
	}
}
