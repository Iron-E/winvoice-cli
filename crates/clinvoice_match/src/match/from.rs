use super::Match;

impl<T> From<T> for Match<T>
{
	fn from(t: T) -> Self
	{
		Self::EqualTo(t)
	}
}
