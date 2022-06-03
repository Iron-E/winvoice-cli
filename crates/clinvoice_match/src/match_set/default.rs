use super::MatchSet;

impl<T> Default for MatchSet<T>
{
	fn default() -> Self
	{
		Self::Any
	}
}
