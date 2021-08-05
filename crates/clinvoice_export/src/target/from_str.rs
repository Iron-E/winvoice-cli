use core::str::FromStr;

use super::{Error, Result, Target};

impl FromStr for Target
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		let lowercase = s.to_ascii_lowercase();
		match lowercase.as_str()
		{
			"markdown" => Ok(Self::Markdown),
			_ => Err(Error::InvalidTarget(lowercase)),
		}
	}
}
