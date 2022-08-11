use core::str::FromStr;

use super::FlagOrArgument;

impl<T> FromStr for FlagOrArgument<T>
where
	T: FromStr,
{
	type Err = T::Err;

	/// NOTE: this interprets `--foo true` and `--foo false` as boolean values, not filepaths.
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		if let Ok(b) = s.parse()
		{
			return Ok(Self::Flag(b));
		}

		s.parse().map(Self::Argument)
	}
}
