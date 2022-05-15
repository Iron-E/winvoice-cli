use core::fmt::{Display, Formatter, Result};

use super::ContactKind;

impl Display for ContactKind
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			ContactKind::Address(ref location) => write!(formatter, "{location}"),
			ContactKind::Email(ref s) | ContactKind::Phone(ref s) =>
			{
				write!(formatter, "{s}")
			},
		}
	}
}
