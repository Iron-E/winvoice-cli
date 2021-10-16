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
	use std::time::Instant;

	use super::EmployeeStatus;

	#[test]
	fn display()
	{
		let start = Instant::now();
		assert_eq!(format!("{}", EmployeeStatus::Employed), "Employed");
		assert_eq!(format!("{}", EmployeeStatus::NotEmployed), "Not employed");
		assert_eq!(
			format!("{}", EmployeeStatus::Representative),
			"Representative"
		);
		println!(
			"\n>>>>> EmployeeStatus::fmt {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros() / 3
		);
	}
}
