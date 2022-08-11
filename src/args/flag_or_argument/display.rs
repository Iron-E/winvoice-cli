use core::fmt::{Debug, Display, Formatter, Result};
use std::path::PathBuf;

use super::FlagOrArgument;

impl Display for FlagOrArgument<PathBuf>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Argument(arg) => Debug::fmt(arg, f),
			Self::Flag(flag) => Display::fmt(flag, f),
		}
	}
}
