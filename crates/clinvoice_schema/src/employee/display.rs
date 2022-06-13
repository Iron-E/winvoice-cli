use core::fmt::{Display, Formatter, Result};

use super::Employee;

impl Display for Employee
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} {}", self.title, self.name)?;
		write!(formatter, "\tStatus: {}", self.status)
	}
}

#[cfg(test)]
mod tests
{
	use super::Employee;

	#[test]
	fn display()
	{
		let employee = Employee {
			id: 0,
			name: "Testy McTesterson".into(),
			status: "Representative".into(),
			title: "CEO of Tests".into(),
		};

		assert_eq!(
			format!("{employee}"),
			"CEO of Tests Testy McTesterson
	Status: Representative",
		);
	}
}
