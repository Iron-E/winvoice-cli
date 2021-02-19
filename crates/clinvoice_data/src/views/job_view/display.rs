use
{
	super::JobView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for JobView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{:?}", self);
	}
}


