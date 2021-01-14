use super::AdapterMismatchError;

use core::fmt::{Display, Formatter, Result as FmtResult};

impl Display for AdapterMismatchError<'_>
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		write!(formatter, "{}", self.message)
	}
}
