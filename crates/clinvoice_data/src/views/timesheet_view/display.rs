use
{
	super::TimesheetView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for TimesheetView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{:?}", self);
	}
}

