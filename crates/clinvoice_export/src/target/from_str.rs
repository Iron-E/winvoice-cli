use
{
	core::str::FromStr,

	super::{Error, Result, Target},
};

impl FromStr for Target
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		todo!()
	}
}
