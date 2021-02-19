use
{
	super::Money,
	std::fmt::{Display, Formatter, Result},
};

impl Display for Money
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{} {}", self.amount, self.currency);
	}
}

