use core::fmt::{Display, Formatter, Result};

use super::ContactKind;

impl Display for ContactKind
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			Self::Address(ref location) => location.fmt(formatter),
			Self::Email(ref s) | Self::Other(ref s) | Self::Phone(ref s) =>
			{
				s.fmt(formatter)
			},
		}
	}
}
