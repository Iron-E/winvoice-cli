use core::fmt::{Display, Formatter, Result};

use super::PgTypeCast;

impl<TCast, TColumn> Display for PgTypeCast<TCast, TColumn>
where
	TCast: Display,
	TColumn: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}::{}", self.0, self.1)
	}
}
