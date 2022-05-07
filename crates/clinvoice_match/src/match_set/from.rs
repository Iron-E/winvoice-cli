use core::fmt::Debug;

use super::MatchSet;

impl<T> From<T> for MatchSet<T>
where
	T: Clone + Debug,
{
	fn from(t: T) -> Self
	{
		Self::Contains(t)
	}
}
