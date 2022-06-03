use super::MatchSet;

impl<T> From<T> for MatchSet<T>
{
	fn from(t: T) -> Self
	{
		Self::Contains(t)
	}
}
