use core::fmt::{Display, Formatter, Result};

use super::PostgresOption;

impl<D> Display for PostgresOption<D>
where
	D: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		match self.0
		{
			Some(ref s) => write!(f, "{}", s),
			_ => write!(f, "NULL"),
		}
	}
}
