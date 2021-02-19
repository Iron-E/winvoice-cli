use
{
	super::ContactView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for ContactView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{:?}", self);
	}
}
