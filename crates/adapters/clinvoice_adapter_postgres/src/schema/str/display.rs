use core::fmt::{Display, Formatter, Result};

use super::PgStr;

impl Display for PgStr<'_>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "'{}'", self.0)
	}
}
