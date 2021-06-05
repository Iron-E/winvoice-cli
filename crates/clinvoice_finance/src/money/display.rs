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
		crate::Currency,
	};

	#[test]
	fn display()
	{
		let start = Instant::now();
		assert_eq!(format!("{}", Money::new(50_00, 2, Currency::USD)), "50.00 USD");
		assert_eq!(format!("{}", Money::new(90_00, 2, Currency::EUR)), "90.00 EUR");
		assert_eq!(format!("{}", Money::new(20000, 0, Currency::JPY)), "20000 JPY");
		println!("\n>>>>> Money::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 3);
	}
}
