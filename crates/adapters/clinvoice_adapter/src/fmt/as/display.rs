use core::fmt::{Display, Formatter, Result};

use super::As;

impl<TAlias, TColumn> Display for As<TAlias, TColumn>
where
	TAlias: Display,
	TColumn: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{} as {}", self.0, self.1)
	}
}
