use
{
	super::Expense,
	std::fmt::{Display, Formatter, Result},
};

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
	use
	{
		super::Expense,
		crate::{Decimal, ExpenseCategory, Money},
		std::time::Instant,
	};

	#[test]
	fn display()
	{
		let expense = Expense
		{
			category: ExpenseCategory::Food,
			cost: Money::new(Decimal::new(2000, 2), "USD"),
			description: "Take-out for 2".into(),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", expense),
"Food – 20.00 USD
	Take-out for 2",
		);
		println!("\n>>>>> Expense::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
