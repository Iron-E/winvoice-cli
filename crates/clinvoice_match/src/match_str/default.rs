use super::MatchStr;

impl<S> Default for MatchStr<S>
{
	fn default() -> Self
	{
		Self::Any
	}
}
