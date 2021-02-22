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

#[cfg(test)]
mod tests
{
	use
	{
		super::EmployeeStatus,
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let start = Instant::now();

		assert_eq!(format!("{}", EmployeeStatus::Employed), "Employed");
		assert_eq!(format!("{}", EmployeeStatus::NotEmployed), "Not employed");
		assert_eq!(format!("{}", EmployeeStatus::Representative), "Representative");

		println!("\n>>>>> EmployeeStatus test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
