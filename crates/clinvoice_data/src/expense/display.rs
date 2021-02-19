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
		return write!(formatter, "{}", self.description);
	}
}
