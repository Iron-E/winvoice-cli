use core::fmt::{Display, Formatter, Result};

use super::TypeCast;

impl<TCast, TColumn> Display for TypeCast<TCast, TColumn>
where
	TCast: Display,
	TColumn: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "CAST({} AS {})", self.0, self.1)
	}
}
