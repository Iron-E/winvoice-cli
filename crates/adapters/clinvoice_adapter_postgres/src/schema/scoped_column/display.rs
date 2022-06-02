use core::fmt::{Display, Formatter, Result};

use super::PgScopedColumn;

impl<TColumn, TIdent> Display for PgScopedColumn<TColumn, TIdent>
where
	TColumn: Display,
	TIdent: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "{}.{}", self.0, self.1)
	}
}
