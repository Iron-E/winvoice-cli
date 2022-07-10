use core::fmt::{Display, Formatter, Result};

use super::As;
use crate::fmt::sql;

impl<TAs, TIdent> Display for As<TIdent, TAs>
where
	TAs: Display,
	TIdent: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}{}{}", self.0, sql::AS, self.1)
	}
}
