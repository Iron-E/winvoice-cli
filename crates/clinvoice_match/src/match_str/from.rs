use super::MatchStr;

impl<T> From<T> for MatchStr<T>
{
	fn from(t: T) -> Self
	{
		Self::EqualTo(t)
	}
}
