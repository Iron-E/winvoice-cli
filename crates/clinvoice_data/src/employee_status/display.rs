use
{
	core::fmt::{Display, Formatter, Result},

	super::EmployeeStatus,
};

impl Display for EmployeeStatus
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}", match self
		{
			EmployeeStatus::Employed => "Employed",
			EmployeeStatus::NotEmployed => "Not employed",
			EmployeeStatus::Representative => "Representative",
		})
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::EmployeeStatus,
	};

	#[test]
	fn display()
	{
		let start = Instant::now();
		assert_eq!(format!("{}", EmployeeStatus::Employed), "Employed");
		assert_eq!(format!("{}", EmployeeStatus::NotEmployed), "Not employed");
		assert_eq!(format!("{}", EmployeeStatus::Representative), "Representative");
		println!("\n>>>>> EmployeeStatus::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 3);
	}
}
