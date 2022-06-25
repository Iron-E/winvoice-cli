use core::fmt::{Display, Formatter, Result};

use super::Expense;

impl Display for Expense
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(
			formatter,
			"№{} – {} ({})",
			self.id, self.category, self.cost
		)?;
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
			id: 10,
			category: "Food".into(),
			cost: Money::new(20_00, 2, Currency::Usd),
			description: "Take-out for 2".into(),
			..Default::default()
		};

		assert_eq!(
			format!("{expense}"),
			"№10 – Food (20.00 USD)
	Take-out for 2",
		);
	}
}
