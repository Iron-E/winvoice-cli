use
{
	super::Expense,
	std::fmt::{Display, Formatter, Result},
};

impl Display for Expense
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "{} – {}", self.category, self.cost)?;
		return write!(formatter, "\t{}", self.description.replace('\n', "\n\t"));
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::Expense,
		crate::{Decimal, ExpenseCategory, Money},
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let start = Instant::now();

		let expense = Expense
		{
			category: ExpenseCategory::Food,
			cost: Money::new(Decimal::new(2000, 2), "USD"),
			description: "Take-out for 2.".into(),
		};

		assert_eq!(
			format!("{}", expense),
"Food – 20.00 USD
	Take-out for 2.",
		);

		println!("\n>>>>> Expense test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
