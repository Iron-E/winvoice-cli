use core::fmt::{Display, Formatter, Result};

use super::PgTypeCast;

impl<D> Display for PgTypeCast<D>
where
	D: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}::{}", self.0, self.1)
	}
}
