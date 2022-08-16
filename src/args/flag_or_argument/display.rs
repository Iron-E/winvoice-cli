use core::fmt::{Debug, Display, Formatter, Result};
use std::path::PathBuf;

use clinvoice_schema::chrono::NaiveDateTime;

use super::FlagOrArgument;

impl Display for FlagOrArgument<PathBuf>
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		match self
		{
			Self::Argument(arg) => Debug::fmt(arg, f),
			Self::Flag(flag) => Display::fmt(flag, f),
		}
	}
}

impl Display for FlagOrArgument<NaiveDateTime>
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		match self
		{
			Self::Argument(arg) => Display::fmt(arg, f),
			Self::Flag(flag) => Display::fmt(flag, f),
		}
	}
}
