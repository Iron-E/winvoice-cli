use core::fmt::{Display, Formatter, Result};

use super::PgLocationRecursiveCte;

impl<T, TOuter> Display for PgLocationRecursiveCte<T, TOuter>
where
	T: Display,
	TOuter: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		self.0.fmt(f)
	}
}
