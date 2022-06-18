use core::fmt::{Display, Result};
use std::fmt::Formatter;

use super::PgLocationRecursiveCte;

impl<TCurrent, TInner> Display for PgLocationRecursiveCte<TCurrent, TInner>
where
	TCurrent: Display,
	TInner: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		self.0.fmt(f)
	}
}
