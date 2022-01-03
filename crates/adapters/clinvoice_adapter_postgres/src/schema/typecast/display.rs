use core::fmt::{Display, Formatter, Result};

use super::PostgresTypeCast;

impl<D> Display for PostgresTypeCast<D>
where
	D: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}::{}", self.0, self.1)
	}
}

