use core::fmt::{Display, Formatter, Result};

use super::WithIdentifier;

impl<TColumn, TIdent> Display for WithIdentifier<TColumn, TIdent>
where
	TColumn: Display,
	TIdent: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}.{}", self.0, self.1)
	}
}
