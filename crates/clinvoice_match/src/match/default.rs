use core::fmt::Debug;

use super::Match;

impl<T> Default for Match<'_, T>
where
	T: Clone + Debug
{
	fn default() -> Self
	{
		Self::Any
	}
}
