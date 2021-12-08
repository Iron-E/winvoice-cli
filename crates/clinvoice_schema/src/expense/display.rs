use core::fmt::{Display, Formatter, Result};

use super::Expense;

impl Display for Expense
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} – {}", self.category, self.cost)?;
		write!(formatter, "\t{}", self.description.replace('\n', "\n\t"))
	}
}

#[cfg(test)]
mod tests
{
	use clinvoice_finance::{Currency, Money};

	use super::Expense;

	#[test]
	fn display()
	{
		let expense = Expense {
			category: "Food".into(),
			cost: Money::new(20_00, 2, Currency::USD),
			description: "Take-out for 2".into(),
		};

		assert_eq!(
			format!("{}", expense),
			"Food – 20.00 USD
	Take-out for 2",
		);
	}
}
