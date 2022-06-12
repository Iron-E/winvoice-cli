use super::MatchRow;

impl<T> From<T> for MatchRow<T>
{
	fn from(t: T) -> Self
	{
		Self::EqualTo(t)
	}
}
