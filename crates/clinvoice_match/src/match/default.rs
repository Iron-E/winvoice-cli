use super::Match;

impl<T> Default for Match<T>
{
	fn default() -> Self
	{
		Self::Any
	}
}
