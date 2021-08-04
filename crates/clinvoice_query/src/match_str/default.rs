use super::MatchStr;

impl<S> Default for MatchStr<S>
where
	S: AsRef<str>,
{
	fn default() -> Self
	{
		Self::Any
	}
}
