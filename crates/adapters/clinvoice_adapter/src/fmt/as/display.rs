use core::fmt::{Display, Formatter, Result};

use super::As;
use crate::fmt::sql;

impl<TAlias, TColumn> Display for As<TAlias, TColumn>
where
	TAlias: Display,
	TColumn: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}{}{}", self.0, sql::AS, self.1)
	}
}
