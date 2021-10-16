use core::fmt::{Display, Formatter, Result};

use super::Currency;

impl Display for Currency
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}", self.as_str())
	}
}
