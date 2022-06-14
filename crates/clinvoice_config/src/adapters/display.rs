use core::fmt::{Display, Formatter, Result};

use super::Adapters;

impl Display for Adapters
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			Adapters::Postgres => "Postgres".fmt(formatter),
		}
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