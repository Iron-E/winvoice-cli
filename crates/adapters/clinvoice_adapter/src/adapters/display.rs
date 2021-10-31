use core::fmt::{Display, Formatter, Result as FmtResult};

use super::Adapters;

impl Display for Adapters
{
	fn fmt(&self, formatter: &mut Formatter) -> FmtResult
	{
		write!(formatter, "{}", match self
		{
			Adapters::Postgres => "Postgres",
		})
	}
}

#[cfg(test)]
mod tests
{
	use super::Adapters;

	#[test]
	fn display()
	{
		assert_eq!(format!("{}", Adapters::Postgres), "Postgres");
	}
}
