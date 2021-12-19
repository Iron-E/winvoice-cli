use core::fmt::{Display, Formatter, Result};

use super::PostgresStr;

impl Display for PostgresStr<'_>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "'{}'", self.0)
	}
}
