use core::fmt::{Display, Formatter, Result};

use super::PgTypeCast;

impl<TCast, TIdent> Display for PgTypeCast<TCast, TIdent>
where
	TIdent: Display, TCast: Display
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}::{}", self.0, self.1)
	}
}
