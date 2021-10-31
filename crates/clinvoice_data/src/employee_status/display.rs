use core::fmt::{Display, Formatter, Result};

use super::EmployeeStatus;

impl Display for EmployeeStatus
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}", self.as_str())
	}
}

#[cfg(test)]
mod tests
{
	use super::EmployeeStatus;

	#[test]
	fn display()
	{
		assert_eq!(format!("{}", EmployeeStatus::Employed), "Employed");
		assert_eq!(format!("{}", EmployeeStatus::NotEmployed), "Not employed");
		assert_eq!(
			format!("{}", EmployeeStatus::Representative),
			"Representative"
		);
	}
}
