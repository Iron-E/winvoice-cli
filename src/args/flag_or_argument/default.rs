use super::FlagOrArgument;

impl<Unused> Default for FlagOrArgument<Unused>
{
	fn default() -> Self
	{
		Self::Flag(bool::default())
	}
}
