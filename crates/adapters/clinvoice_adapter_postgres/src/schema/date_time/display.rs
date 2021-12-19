use core::fmt::{Display, Formatter, Result};

use super::PostgresDateTime;

impl Display for PostgresDateTime
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "TIMESTAMP WITH TIME ZONE '{}'", self.0)
	}
}
