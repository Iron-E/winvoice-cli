use core::fmt::Debug;

use super::Match;

impl<T> Default for Match<T>
where
	T: Clone + Debug,
{
	fn default() -> Self
	{
		Self::Any
	}
}
