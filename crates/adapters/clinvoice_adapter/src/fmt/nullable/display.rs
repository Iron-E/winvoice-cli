use core::fmt::{Display, Formatter, Result};

use super::Nullable;
use crate::fmt::sql;

impl<D> Display for Nullable<D>
where
	D: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		match self.0
		{
			Some(ref s) => write!(f, "{s}"),
			_ => sql::NULL.fmt(f),
		}
	}
}
