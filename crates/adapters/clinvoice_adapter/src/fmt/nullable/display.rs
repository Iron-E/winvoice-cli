use core::fmt::{Display, Formatter, Result};

use super::Nullable;

impl<D> Display for Nullable<D>
where
	D: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		match self.0
		{
			Some(ref s) => write!(f, "{s}"),
			_ => write!(f, "NULL"),
		}
	}
}