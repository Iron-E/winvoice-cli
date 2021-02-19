use
{
	super::LocationView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for LocationView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{:?}", self);
	}
}
