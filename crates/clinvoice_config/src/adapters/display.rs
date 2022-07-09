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
	use pretty_assertions::assert_eq;

	use super::Adapters;

	#[test]
	fn display()
	{
		assert_eq!(Adapters::Postgres.to_string(), "Postgres");
	}
}
