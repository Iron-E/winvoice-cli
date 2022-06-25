use core::fmt::{Display, Formatter, Result};

use super::TypeCast;
use crate::fmt::{sql, As};

impl<TCast, TColumn> Display for TypeCast<TCast, TColumn>
where
	TCast: Display,
	TColumn: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}({})", sql::CAST, As(&self.0, &self.1))
	}
}
