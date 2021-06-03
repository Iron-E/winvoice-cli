use
{
	core::fmt::{Display, Formatter, Result},

	super::Money,
};

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
	use
	{
		std::time::Instant,

		super::Money,
		crate::Decimal,
	};

	#[test]
	fn display()
	{
		let start = Instant::now();
		assert_eq!(format!("{}", Money::new(Decimal::new(5000, 2), "USD")), "50.00 USD");
		assert_eq!(format!("{}", Money::new(Decimal::new(9000, 2), "EUR")), "90.00 EUR");
		assert_eq!(format!("{}", Money::new(Decimal::new(20000, 0), "JPY")), "20000 JPY");
		println!("\n>>>>> Money::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 3);
	}
}
