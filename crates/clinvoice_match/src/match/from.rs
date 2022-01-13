use core::fmt::Debug;

use super::Match;

impl<T> From<T> for Match<T>
where
	T: Clone + Debug,
{
	fn from(t: T) -> Self
	{
		Self::EqualTo(t)
	}
}
