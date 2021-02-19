use
{
	super::EmployeeView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for EmployeeView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return writeln!(formatter, "{:?}", self);
	}
}

