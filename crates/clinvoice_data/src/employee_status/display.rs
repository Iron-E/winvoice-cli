use
{
	super::EmployeeStatus,
	std::fmt::{Display, Formatter, Result},
};

impl Display for EmployeeStatus
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{}", match self
		{
			EmployeeStatus::Employed => "Employed",
			EmployeeStatus::NotEmployed => "Not employed",
			EmployeeStatus::Representative => "Representative",
		});
	}
}


