use core::fmt::{Display, Formatter, Result};

use super::Command;

impl Display for Command
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Start => write!(formatter, "start"),
			Self::Stop => write!(formatter, "stop"),
		}
	}
}
